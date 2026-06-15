"""synthesize_news.py — date logic, track routing, generation via FakeAnthropicClient."""
import json
from datetime import datetime, timezone

import synthesize_news as sn


# ─── date helpers ─────────────────────────────────────────────────────────────
def test_isoweek_id_format():
    dt = datetime(2026, 6, 15, tzinfo=timezone.utc)  # a Monday
    assert sn.isoweek_id(dt).startswith("synthesis_2026_W")


def test_parse_week_roundtrip():
    start, end = sn.parse_week("2026-W25")
    assert start.weekday() == 0          # Monday
    assert (end - start).days == 6


def test_ai_window_uses_last_ai_period_end():
    data = {"items": [
        {"type": "synthesis", "track": "ai", "period_end": "2026-06-12"},
        {"type": "synthesis", "track": "general", "period_end": "2026-06-14"},
    ]}
    now = datetime(2026, 6, 15, 6, 0, 0, tzinfo=timezone.utc)
    start, end, sid = sn.ai_window(data, now)
    assert start.strftime("%Y-%m-%d") == "2026-06-13"   # day after last ai end
    assert sid == "synthesis_ai_2026-06-15"


def test_ai_window_defaults_to_7_days_when_no_prior():
    data = {"items": []}
    now = datetime(2026, 6, 15, 6, 0, 0, tzinfo=timezone.utc)
    start, end, sid = sn.ai_window(data, now)
    assert start.strftime("%Y-%m-%d") == "2026-06-08"


# ─── cap_articles ─────────────────────────────────────────────────────────────
def test_cap_articles_limits_total_and_picks_top_scores():
    arts = [{"id": f"x{i}", "domain": "dev_stack", "score": float(i)} for i in range(20)]
    out = sn.cap_articles(arts, limit=8, per_domain=8)
    assert len(out) == 8
    assert out[0]["score"] == 19.0           # highest first
    assert min(a["score"] for a in out) == 12.0


def test_cap_articles_respects_per_domain():
    arts = ([{"id": f"d{i}", "domain": "dev_stack", "score": 9.0 - i} for i in range(6)]
            + [{"id": f"s{i}", "domain": "security", "score": 5.0 - i} for i in range(6)])
    out = sn.cap_articles(arts, limit=8, per_domain=3)
    assert sum(1 for a in out if a["domain"] == "dev_stack") <= 3
    assert sum(1 for a in out if a["domain"] == "security") <= 3


def test_cap_articles_handles_missing_score():
    arts = [{"id": "a", "domain": "ai"}, {"id": "b", "domain": "ai", "score": 7.0}]
    out = sn.cap_articles(arts, limit=8, per_domain=8)
    assert out[0]["id"] == "b"               # scored one ranks above unscored


# ─── build_context ────────────────────────────────────────────────────────────
def test_build_context_groups_by_domain():
    items = [{"id": "d1", "source": "S", "lang": "en", "title": "T", "domain": "ai"}]
    ctx = sn.build_context(items)
    assert "AI" in ctx and "T" in ctx
    assert "srcref:d1" in ctx          # link token exposed to the model


# ─── source-link resolution ─────────────────────────────────────────────────────
def test_resolve_source_links_link_form():
    content = "See [the release](srcref:d1) and [the patch](srcref:s1)."
    out = sn.resolve_source_links(content, {"d1": "https://a.com/x", "s1": "https://b.com/y"})
    assert "[the release](https://a.com/x)" in out
    assert "[the patch](https://b.com/y)" in out
    assert "srcref:" not in out


def test_resolve_source_links_bare_token():
    out = sn.resolve_source_links("ref srcref:d1 inline", {"d1": "https://a.com/x"})
    assert "https://a.com/x" in out
    assert "srcref:" not in out


def test_resolve_source_links_unknown_id_untouched():
    content = "[x](srcref:zzz)"
    out = sn.resolve_source_links(content, {"d1": "https://a.com/x"})
    assert out == content          # unknown id left as-is, never a wrong URL


def test_resolve_source_links_empty():
    assert sn.resolve_source_links("", {"d1": "u"}) == ""


# ─── illustration gating ─────────────────────────────────────────────────────────
def test_is_generic_image_flags_envelope_and_logo():
    assert sn.is_generic_image("https://x.com/assets/envelope.png")
    assert sn.is_generic_image("https://x.com/default-share.jpg")
    assert sn.is_generic_image("https://x.com/logo.svg")
    assert not sn.is_generic_image("https://res.infoq.com/news/header/realimg.jpg")


def test_vet_illustration_keep(fake_client):
    ill = {"url": "https://x.com/screenshot.png", "caption": "Vite 8 UI"}
    assert sn.vet_illustration(fake_client(default="KEEP"), ill, "ctx") == ill


def test_vet_illustration_drop(fake_client):
    ill = {"url": "https://x.com/whatever.png", "caption": "x"}
    assert sn.vet_illustration(fake_client(default="DROP"), ill, "ctx") is None


def test_vet_illustration_none_when_empty(fake_client):
    assert sn.vet_illustration(fake_client(default="KEEP"), {}, "ctx") is None


# ─── validate_images ──────────────────────────────────────────────────────────
def test_validate_images_removes_flagged(fake_client):
    content = "Intro\n\n![one](https://a.com/1.jpg)\n\nmid\n\n![two](https://b.com/2.jpg)\n"
    client = fake_client(default="KEEP\nREMOVE")
    out = sn.validate_images(client, content, "ctx")
    assert "a.com/1.jpg" in out
    assert "b.com/2.jpg" not in out


def test_validate_images_noop_without_images(fake_client):
    client = fake_client(default="whatever")
    assert sn.validate_images(client, "no images here", "ctx") == "no images here"
    assert len(client.calls) == 0


# ─── generate_ai_synthesis ────────────────────────────────────────────────────
def test_generate_ai_synthesis_english_only(fake_client):
    canned = json.dumps({"title_en": "Frontier shift", "content_en": "## What happened\nStuff."})
    client = fake_client(default=canned)
    arts = [{"source": "OpenAI", "lang": "en", "title": "x", "domain": "ai"}]
    out = sn.generate_ai_synthesis(client, arts, "2026-06-08", "2026-06-15", [], "synthesis_ai_2026-06-15")
    assert out["title_en"] == "Frontier shift"
    assert "content_fr" not in out
    assert len(client.calls) == 1
    # token-lean: small cap, sonnet
    assert client.calls[0]["max_tokens"] <= 4000


def test_generate_ai_synthesis_handles_failure(fake_client):
    client = fake_client(raise_exc=RuntimeError("boom"))
    out = sn.generate_ai_synthesis(client, [{"source": "s", "lang": "en", "title": "x", "domain": "ai"}],
                                   "a", "b", [], "id")
    assert out is None


# ─── main: AI track (lean, no images / no Haiku) ──────────────────────────────
def _ai_items():
    return [
        {"id": "a1", "type": "article", "domain": "ai", "status": "selected",
         "title": "OpenAI frontier model", "url": "https://o.ai/1", "source": "OpenAI",
         "lang": "en", "published_at": "2026-06-14T08:00:00+00:00", "score": 9.0},
        {"id": "a2", "type": "article", "domain": "ai", "status": "selected",
         "title": "EU AI Act update", "url": "https://eu/1", "source": "EU",
         "lang": "en", "published_at": "2026-06-14T09:00:00+00:00", "score": 8.0},
        {"id": "d1", "type": "article", "domain": "dev_stack", "status": "selected",
         "title": "Rust release", "url": "https://r/1", "source": "Rust",
         "lang": "en", "published_at": "2026-06-14T09:00:00+00:00", "score": 8.0},
    ]


def test_main_ai_track_writes_english_only_card(monkeypatch, write_news, fake_client):
    path = write_news(_ai_items())
    canned = json.dumps({"title_en": "AI Brief", "content_en": "## What happened\n" + "word " * 350})
    client = fake_client(default=canned)
    monkeypatch.setattr(sn.anthropic, "Anthropic", lambda api_key=None: client)
    monkeypatch.setattr("sys.argv", ["synthesize_news.py", "--track", "ai", "--force"])

    sn.main()

    data = json.loads(path.read_text(encoding="utf-8"))
    card = next(i for i in data["items"] if i.get("type") == "synthesis")
    assert card["track"] == "ai"
    assert card["id"].startswith("synthesis_ai_")
    assert card["content_fr"] == ""              # English only
    assert card["content_en"]
    assert card["illustration"] is None          # no images
    # only the ai-domain articles became sources (dev_stack excluded)
    src_ids = {s["id"] for s in card["sources"]}
    assert src_ids == {"a1", "a2"}
    # token-lean: exactly one model call (no Haiku proofread / image review)
    assert len(client.calls) == 1


# ─── main: general track excludes AI ──────────────────────────────────────────
def test_main_general_track_excludes_ai(monkeypatch, write_news, fake_client):
    # ISO week 2026-W25 = Mon 2026-06-15 → Sun 2026-06-21
    items = [
        {"id": "a1", "type": "article", "domain": "ai", "status": "selected",
         "title": "OpenAI frontier model", "url": "https://o.ai/1", "source": "OpenAI",
         "lang": "en", "published_at": "2026-06-16T08:00:00+00:00", "score": 9.0},
        {"id": "d1", "type": "article", "domain": "dev_stack", "status": "selected",
         "title": "Rust release", "url": "https://r/1", "source": "Rust",
         "lang": "en", "published_at": "2026-06-16T09:00:00+00:00", "score": 8.0},
        {"id": "s1", "type": "article", "domain": "security", "status": "selected",
         "title": "CVE patch", "url": "https://s/1", "source": "CISA",
         "lang": "en", "published_at": "2026-06-16T09:00:00+00:00", "score": 8.0},
        {"id": "b1", "type": "article", "domain": "business_market", "status": "selected",
         "title": "Market shift", "url": "https://b/1", "source": "InfoQ",
         "lang": "en", "published_at": "2026-06-16T09:00:00+00:00", "score": 7.0},
    ]
    path = write_news(items)

    syn = json.dumps({
        "title_fr": "FR", "title_en": "EN",
        "content_fr": "fr " * 1200, "content_en": "en text",
        "security_actions": [], "architecture_info": {"present": False},
    })

    def responder(kwargs):
        p = kwargs["messages"][0]["content"]
        if "KEEP" in p or "KEEP ou REMOVE" in p:
            return "KEEP"
        if "correcteur de style" in p:           # proofread pass
            return "fr " * 1200
        return syn

    client = fake_client(responder=responder)
    monkeypatch.setattr(sn.anthropic, "Anthropic", lambda api_key=None: client)
    monkeypatch.setattr(sn, "find_images", lambda arts, max_images=3: [])  # no network
    monkeypatch.setattr("sys.argv", ["synthesize_news.py", "--track", "general", "--force",
                                     "--week", "2026-W25"])

    sn.main()

    data = json.loads(path.read_text(encoding="utf-8"))
    card = next(i for i in data["items"] if i.get("type") == "synthesis")
    assert card["track"] == "general"
    src_domains = {s["domain"] for s in card["sources"]}
    assert "ai" not in src_domains               # AI excluded from general
    assert src_domains <= {"security", "business_market", "dev_stack"}
