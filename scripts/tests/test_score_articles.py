"""score_articles.py — selection logic (pure) + scoring via FakeAnthropicClient."""
import json

import score_articles as sa


def _winner(id_, domain, score):
    return {"id": id_, "domain": domain, "title": id_, "source": "s", "_score": score}


# ─── select_global_top5 ───────────────────────────────────────────────────────
def test_general_top_excludes_ai():
    winners = {
        "dev_stack": [_winner("d1", "dev_stack", 6.0)],
        "ai": [_winner("a1", "ai", 9.9), _winner("a2", "ai", 9.8)],
        "security": [_winner("s1", "security", 7.0)],
    }
    selected = sa.select_global_top5(winners)
    assert "a1" not in selected and "a2" not in selected
    assert "d1" in selected and "s1" in selected


def test_general_top_caps_architecture():
    winners = {
        "architecture": [_winner("ar1", "architecture", 9.0),
                         _winner("ar2", "architecture", 8.9),
                         _winner("ar3", "architecture", 8.8)],
        "dev_stack": [_winner("d1", "dev_stack", 5.0)],
    }
    selected = sa.select_global_top5(winners)
    arch = [s for s in selected if s.startswith("ar")]
    assert len(arch) <= sa.MAX_ARCH_IN_TOP


def test_general_top_size_capped():
    winners = {"dev_stack": [_winner(f"d{i}", "dev_stack", 9 - i) for i in range(10)]}
    assert len(sa.select_global_top5(winners)) <= sa.GLOBAL_TOP


# ─── select_ai_top ────────────────────────────────────────────────────────────
def test_ai_top_picks_highest_ai_only():
    winners = {
        "ai": [_winner("a1", "ai", 5.0), _winner("a2", "ai", 9.0), _winner("a3", "ai", 7.0)],
        "dev_stack": [_winner("d1", "dev_stack", 10.0)],
    }
    top = sa.select_ai_top(winners, n=2)
    assert top == ["a2", "a3"]
    assert "d1" not in top


def test_ai_top_empty_when_no_ai():
    assert sa.select_ai_top({"dev_stack": [_winner("d1", "dev_stack", 5.0)]}) == []


# ─── score_domain (fake client) ───────────────────────────────────────────────
def test_score_domain_parses_and_sorts(fake_client):
    arts = [{"id": "x", "title": "X", "source": "s"},
            {"id": "y", "title": "Y", "source": "s"}]
    canned = json.dumps([
        {"id": "x", "score": 3.0, "reason": "meh"},
        {"id": "y", "score": 8.0, "reason": "great"},
    ])
    client = fake_client(default=canned)
    out = sa.score_domain(client, "ai", arts)
    assert out[0]["id"] == "y"           # sorted desc
    assert out[0]["_score"] == 8.0
    assert out[1]["_score"] == 3.0
    assert len(client.calls) == 1        # exactly one API call


def test_score_domain_strips_markdown_fence(fake_client):
    arts = [{"id": "x", "title": "X", "source": "s"}]
    canned = "```json\n" + json.dumps([{"id": "x", "score": 5.0, "reason": "ok"}]) + "\n```"
    out = sa.score_domain(fake_client(default=canned), "ai", arts)
    assert out[0]["_score"] == 5.0


def test_score_domain_falls_back_on_api_error(fake_client):
    arts = [{"id": "x", "title": "X", "source": "s"},
            {"id": "y", "title": "Y", "source": "s"}]
    client = fake_client(raise_exc=RuntimeError("boom"))
    out = sa.score_domain(client, "ai", arts)
    assert all("_score" in a for a in out)
    assert all(a["_score_reason"] == "fallback order" for a in out)


def test_ai_criteria_defined():
    assert "ai" in sa.DOMAIN_CRITERIA
    assert "ai_emerging" not in sa.DOMAIN_CRITERIA


# ─── main (offline integration) ───────────────────────────────────────────────
def test_main_selects_general_and_ai_separately(monkeypatch, write_news, fake_client):
    items = [
        {"id": "d1", "type": "article", "domain": "dev_stack", "status": "raw",
         "title": "Rust release", "source": "Rust", "lang": "en",
         "published_at": "2026-06-15T08:00:00+00:00", "fetched_at": "2026-06-15T08:00:00+00:00"},
        {"id": "a1", "type": "article", "domain": "ai", "status": "raw",
         "title": "OpenAI frontier model", "source": "OpenAI", "lang": "en",
         "published_at": "2026-06-15T08:00:00+00:00", "fetched_at": "2026-06-15T08:00:00+00:00"},
        {"id": "a2", "type": "article", "domain": "ai", "status": "raw",
         "title": "EU AI Act update", "source": "EU", "lang": "en",
         "published_at": "2026-06-15T08:00:00+00:00", "fetched_at": "2026-06-15T08:00:00+00:00"},
    ]
    path = write_news(items)

    # Each scoring call returns scores for whatever ids it was given.
    def responder(kwargs):
        prompt = kwargs["messages"][0]["content"]
        scores = []
        for line in prompt.splitlines():
            if "[ID:" in line:
                aid = line.split("[ID:")[1].split("]")[0]
                scores.append({"id": aid, "score": 8.0, "reason": "ok"})
        return json.dumps(scores)

    monkeypatch.setattr(sa.anthropic, "Anthropic", lambda api_key=None: fake_client(responder=responder))
    monkeypatch.setattr("sys.argv", ["score_articles.py", "--date", "2026-06-15"])
    sa.main()

    data = json.loads(path.read_text(encoding="utf-8"))
    by_id = {it["id"]: it for it in data["items"]}
    assert by_id["d1"]["status"] == "selected"          # general
    assert by_id["a1"]["status"] == "selected"          # ai track
    assert by_id["a2"]["status"] == "selected"
