"""fetch_news.py — pure logic + offline RSS/HN parsing. No network, no tokens."""
import json
from types import SimpleNamespace

import feedparser
import requests

import fetch_news as fn


# ─── classify ─────────────────────────────────────────────────────────────────
def test_classify_urgent():
    assert "urgent" in fn.classify("New CVE-2026-1234 vulnerability disclosed")


def test_classify_good_news():
    assert "good_news" in fn.classify("Tool 1.0 released and generally available")


def test_classify_future_watch_ai():
    assert "future_watch" in fn.classify("A new LLM approach to reasoning")


def test_classify_defaults_to_general():
    assert fn.classify("An ordinary headline about nothing special") == ["general"]


# ─── small helpers ────────────────────────────────────────────────────────────
def test_item_id_is_stable_and_short():
    a = fn.item_id("https://example.com/x")
    b = fn.item_id("https://example.com/x")
    assert a == b and len(a) == 12


def test_is_safe_url():
    assert fn.is_safe_url("https://ok.com")
    assert fn.is_safe_url("http://ok.com")
    assert not fn.is_safe_url("javascript:alert(1)")
    assert not fn.is_safe_url("ftp://x")


def test_noise_filter_matches_low_quality_titles():
    assert fn._NOISE_RE.search("Day 5 of my journey")
    assert fn._NOISE_RE.search("Top 10 things you need to know")
    assert not fn._NOISE_RE.search("Rust 1.90 stabilises const generics")


# ─── retention ────────────────────────────────────────────────────────────────
def test_synthesis_never_expires():
    item = {"status": "synthesis", "published_at": "2000-01-01T00:00:00+00:00"}
    assert fn.is_expired(item, "2026-06-15") is False


def test_old_raw_article_expires():
    item = {"status": "raw", "published_at": "2000-01-01T00:00:00+00:00"}
    assert fn.is_expired(item, "2026-06-15") is True


def test_recent_selected_article_kept():
    from datetime import datetime, timezone
    item = {"status": "selected", "published_at": datetime.now(timezone.utc).isoformat()}
    assert fn.is_expired(item, "2026-06-15") is False


# ─── fetch_rss (offline) ──────────────────────────────────────────────────────
def _fake_feed(entries):
    return SimpleNamespace(entries=entries)


def test_fetch_rss_parses_and_tags_domain(monkeypatch):
    entry = {
        "link": "https://arxiv.org/abs/1",
        "title": "A reproducible result in machine learning",
        "summary": "<p>abstract</p>",
        "published_parsed": (2026, 6, 10, 0, 0, 0, 0, 0, 0),
    }
    monkeypatch.setattr(feedparser, "parse", lambda url: _fake_feed([entry]))
    src = {"name": "ArXiv CS.AI", "url": "x", "lang": "en", "domain": "ai", "no_filter": True}
    items = fn.fetch_rss(src)
    assert len(items) == 1
    assert items[0]["domain"] == "ai"
    assert items[0]["status"] == "raw"
    assert items[0]["source"] == "ArXiv CS.AI"


def test_fetch_rss_applies_noise_filter_when_not_no_filter(monkeypatch):
    entries = [
        {"link": "https://x.com/a", "title": "Top 10 things you need to know"},
        {"link": "https://x.com/b", "title": "Postgres 18 ships logical replication"},
    ]
    monkeypatch.setattr(feedparser, "parse", lambda url: _fake_feed(entries))
    src = {"name": "Blog", "url": "x", "lang": "en", "domain": "dev_stack"}
    titles = [i["title"] for i in fn.fetch_rss(src)]
    assert "Top 10 things you need to know" not in titles
    assert "Postgres 18 ships logical replication" in titles


def test_fetch_rss_no_filter_keeps_everything(monkeypatch):
    entries = [{"link": "https://x.com/a", "title": "Top 10 papers this week"}]
    monkeypatch.setattr(feedparser, "parse", lambda url: _fake_feed(entries))
    src = {"name": "Papers", "url": "x", "lang": "en", "domain": "ai", "no_filter": True}
    assert len(fn.fetch_rss(src)) == 1


# ─── fetch_hn (offline) ───────────────────────────────────────────────────────
def test_fetch_hn_respects_score_threshold(monkeypatch):
    top = [1, 2]
    stories = {
        1: {"type": "story", "url": "https://a.com", "title": "Big news", "score": 500, "time": 1700000000},
        2: {"type": "story", "url": "https://b.com", "title": "Small news", "score": 10, "time": 1700000000},
    }

    def fake_get(url, timeout=10):
        if "topstories" in url:
            return SimpleNamespace(raise_for_status=lambda: None, json=lambda: top)
        sid = int(url.rsplit("/", 1)[1].split(".")[0])
        return SimpleNamespace(raise_for_status=lambda: None, json=lambda: stories[sid])

    monkeypatch.setattr(requests, "get", fake_get)
    items = fn.fetch_hn()
    urls = [i["url"] for i in items]
    assert "https://a.com" in urls
    assert "https://b.com" not in urls  # below MIN_HN_SCORE


# ─── ai sources wired in ──────────────────────────────────────────────────────
def test_ai_domain_present_in_sources():
    domains = {s["domain"] for s in fn.SOURCES}
    assert "ai" in domains
    assert "ai_emerging" not in domains  # migrated away


def test_ai_sources_cover_the_four_categories():
    names = {s["name"] for s in fn.SOURCES if s["domain"] == "ai"}
    # one representative per requested category
    assert {"OpenAI", "Google DeepMind", "Hugging Face"} & names  # labs
    assert "ArXiv CS.AI" in names                                 # research
    assert {"EU Digital Strategy", "NIST News", "ArXiv CS.CY"} & names  # regulatory
    assert {"Import AI", "The Gradient", "Ars Technica AI"} & names     # analysis


# ─── main (offline integration) ───────────────────────────────────────────────
def test_main_writes_valid_schema(monkeypatch, write_news):
    path = write_news([])
    monkeypatch.setattr(fn, "fetch_hn", lambda: [])

    entry = {"link": "https://arxiv.org/abs/1", "title": "OpenAI releases something",
             "summary": "x", "published_parsed": (2026, 6, 14, 0, 0, 0, 0, 0, 0)}
    monkeypatch.setattr(feedparser, "parse", lambda url: _fake_feed([entry]))

    fn.main()
    data = json.loads(path.read_text(encoding="utf-8"))
    assert {"generated_at", "count", "items"} <= data.keys()
    assert data["count"] == len(data["items"])
    assert all("id" in it for it in data["items"])
