"""extract_ledger.py — pure logic only. No git, no network, no tokens."""
import json

import extract_ledger as el


# ─── normalize_domain ───────────────────────────────────────────────────────
def test_normalize_domain_alias():
    assert el.normalize_domain("ai_emerging") == "ai"


def test_normalize_domain_passthrough_and_strip():
    assert el.normalize_domain("  security  ") == "security"
    assert el.normalize_domain("") == ""


# ─── valid_date ──────────────────────────────────────────────────────────────
def test_valid_date():
    assert el.valid_date("2026-06-15T07:00:00") is True
    assert el.valid_date("2026-06-15") is True


def test_invalid_dates_rejected():
    assert el.valid_date("") is False
    assert el.valid_date("0001-01-01") is False   # bogus feed date
    assert el.valid_date("1999-12-31") is False    # pre-2020 floor


# ─── signal_records ──────────────────────────────────────────────────────────
def test_signal_records_from_article():
    news = {"items": [{
        "type": "article", "id": "abc", "title": "Rust 2.0 lands",
        "domain": "ai_emerging", "published_at": "2026-05-01T10:00:00",
        "source": "Rust Blog", "url": "http://x", "score": 7.2,
        "status": "selected", "categories": ["future_watch"], "lang": "en",
    }]}
    recs = el.signal_records(news)
    assert "abc" in recs
    r = recs["abc"]
    assert r["domain"] == "ai"            # alias normalized
    assert r["date"] == "2026-05-01"      # truncated to day
    assert r["tier"] == "tracked"


def test_signal_records_recovers_synthesis_sources():
    news = {"items": [{
        "type": "synthesis", "id": "synth_1",
        "sources": [{"id": "src9", "title": "Old pruned signal",
                     "domain": "security", "published_at": "2026-03-01",
                     "url": "http://y", "score": 8.0, "source": "CISA"}],
    }]}
    recs = el.signal_records(news)
    assert "src9" in recs
    # appearing as a synthesis source implies it cleared selection
    assert recs["src9"]["status"] == "selected"


def test_signal_records_skips_idless():
    news = {"items": [{"type": "article", "title": "no id", "url": ""}]}
    assert el.signal_records(news) == {}


# ─── merge_records (best-record wins) ────────────────────────────────────────
def test_merge_prefers_selected_over_raw():
    raw      = {"x": {"id": "x", "status": "raw", "score": 9.0, "date": "2026-01-01", "domain": "ai", "signal": "s"}}
    selected = {"x": {"id": "x", "status": "selected", "score": 5.0, "date": "2026-01-01", "domain": "ai", "signal": "s"}}
    merged = el.merge_records(raw, selected)
    assert merged["x"]["status"] == "selected"  # status rank beats higher score


def test_merge_prefers_higher_score_at_same_status():
    lo = {"x": {"id": "x", "status": "raw", "score": 3.0, "date": "2026-01-01", "domain": "ai", "signal": "s"}}
    hi = {"x": {"id": "x", "status": "raw", "score": 8.0, "date": "2026-01-01", "domain": "ai", "signal": "s"}}
    assert el.merge_records(lo, hi)["x"]["score"] == 8.0


# ─── qualifies ───────────────────────────────────────────────────────────────
def _rec(**kw):
    base = {"signal": "s", "domain": "ai", "date": "2026-05-01", "status": "raw", "score": 0.0}
    base.update(kw)
    return base


def test_qualifies_selected_always():
    assert el.qualifies(_rec(status="selected", score=0.0),
                        selected_only=False, min_score=6.0, keep_all=False) is True


def test_qualifies_by_score():
    assert el.qualifies(_rec(score=6.5), selected_only=False, min_score=6.0, keep_all=False) is True
    assert el.qualifies(_rec(score=4.0), selected_only=False, min_score=6.0, keep_all=False) is False


def test_qualifies_selected_only_drops_high_score_raw():
    assert el.qualifies(_rec(score=9.0, status="raw"),
                        selected_only=True, min_score=6.0, keep_all=False) is False


def test_qualifies_rejects_missing_fields():
    assert el.qualifies(_rec(domain=""), selected_only=False, min_score=6.0, keep_all=False) is False
    assert el.qualifies(_rec(date="0001-01-01"), selected_only=False, min_score=6.0, keep_all=False) is False
    assert el.qualifies(_rec(signal=""), selected_only=False, min_score=6.0, keep_all=False) is False


def test_keep_all_overrides_score():
    assert el.qualifies(_rec(score=0.0, status="raw"),
                        selected_only=False, min_score=6.0, keep_all=True) is True


# ─── to_ledger_line ──────────────────────────────────────────────────────────
def test_to_ledger_line_shape():
    rec = el.signal_records({"items": [{
        "type": "article", "id": "abc", "title": "T", "domain": "ai",
        "published_at": "2026-05-01", "score": 7.234, "status": "selected",
        "categories": ["urgent"], "url": "http://x", "source": "S",
    }]})["abc"]
    line = el.to_ledger_line(rec)
    assert line["score"] == 7.2           # rounded
    assert line["category"] == "urgent"   # first category
    assert line["tier"] == "tracked"
    assert set(line) == {"date", "domain", "signal", "source", "score",
                         "status", "category", "url", "id", "tier"}


# ─── append_signals (live pipeline hook) ─────────────────────────────────────
def _items(*specs):
    """specs = (id, score, status) tuples → minimal article items."""
    return [{
        "type": "article", "id": i, "title": f"signal {i}", "domain": "ai",
        "published_at": "2026-06-20", "score": sc, "status": st,
        "source": "S", "url": f"http://{i}",
    } for i, sc, st in specs]


def test_append_signals_writes_qualified(tmp_path, monkeypatch):
    monkeypatch.setattr(el, "LEDGER_PATH", tmp_path / "ledger.jsonl")
    added = el.append_signals(_items(("a", 8.0, "selected"), ("b", 2.0, "raw")))
    assert added == 1  # 'b' below threshold and not selected → dropped
    rows = [json.loads(l) for l in (tmp_path / "ledger.jsonl").read_text().splitlines()]
    assert {r["id"] for r in rows} == {"a"}


def test_append_signals_idempotent(tmp_path, monkeypatch):
    monkeypatch.setattr(el, "LEDGER_PATH", tmp_path / "ledger.jsonl")
    el.append_signals(_items(("a", 8.0, "selected")))
    added2 = el.append_signals(_items(("a", 8.0, "selected")))
    assert added2 == 0  # same signal again → no growth
    rows = (tmp_path / "ledger.jsonl").read_text().splitlines()
    assert len(rows) == 1


def test_append_signals_accumulates_new_days(tmp_path, monkeypatch):
    monkeypatch.setattr(el, "LEDGER_PATH", tmp_path / "ledger.jsonl")
    el.append_signals(_items(("a", 8.0, "selected")))
    added = el.append_signals(_items(("c", 7.0, "selected")))
    assert added == 1
    rows = (tmp_path / "ledger.jsonl").read_text().splitlines()
    assert len(rows) == 2
