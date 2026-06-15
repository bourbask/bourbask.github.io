"""
Shared fixtures for the news-pipeline tests.

Design goal: tests must NEVER hit the network and NEVER call the real Anthropic API
(zero token spend). Two guarantees back this up:

1. `no_network` (autouse) makes `requests.get` and `feedparser.parse` raise unless a
   test explicitly monkeypatches its own stub on top — so any accidental real call
   fails loudly instead of silently spending money / flaking.
2. The Anthropic client is dependency-injected everywhere, so tests pass a
   `FakeAnthropicClient` whose `.messages.create()` returns canned text. No API key,
   no request, no tokens.
"""
import sys
from pathlib import Path

import pytest

# Make the scripts importable (they live in scripts/, not a package).
SCRIPTS_DIR = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(SCRIPTS_DIR))


# ─── Fake Anthropic client ────────────────────────────────────────────────────
class _Content:
    def __init__(self, text):
        self.text = text


class _Resp:
    def __init__(self, text):
        self.content = [_Content(text)]


class _FakeMessages:
    def __init__(self, parent):
        self.parent = parent

    def create(self, **kwargs):
        self.parent.calls.append(kwargs)
        if self.parent.raise_exc is not None:
            raise self.parent.raise_exc
        responder = self.parent.responder
        if callable(responder):
            return _Resp(responder(kwargs))
        if self.parent.queue:
            return _Resp(self.parent.queue.pop(0))
        return _Resp(self.parent.default)


class FakeAnthropicClient:
    """Drop-in replacement for anthropic.Anthropic — never touches the network."""

    def __init__(self, responder=None, queue=None, default="", raise_exc=None):
        self.responder = responder
        self.queue = list(queue or [])
        self.default = default
        self.raise_exc = raise_exc
        self.calls = []
        self.messages = _FakeMessages(self)


@pytest.fixture
def fake_client():
    def _make(**kwargs):
        return FakeAnthropicClient(**kwargs)
    return _make


# ─── Network guard ────────────────────────────────────────────────────────────
class NetworkBlocked(RuntimeError):
    pass


@pytest.fixture(autouse=True)
def no_network(monkeypatch):
    """Block all real HTTP / feed parsing. Tests stub their own on top as needed."""
    import requests
    import feedparser

    def _blocked(*a, **k):
        raise NetworkBlocked(
            "Real network call attempted in a test — stub it instead."
        )

    monkeypatch.setattr(requests, "get", _blocked)
    monkeypatch.setattr(feedparser, "parse", _blocked)
    # A dummy key so scripts that check for ANTHROPIC_API_KEY don't bail out.
    # It is never used to authenticate anything (client is faked).
    monkeypatch.setenv("ANTHROPIC_API_KEY", "test-dummy-not-a-real-key")
    yield


# ─── Sample data ──────────────────────────────────────────────────────────────
def make_article(**over):
    art = {
        "id": over.get("id", "abc123"),
        "type": "article",
        "title": "Some technical release announced",
        "url": "https://example.com/post",
        "source": "Example",
        "domain": "dev_stack",
        "categories": ["good_news"],
        "published_at": "2026-06-10T12:00:00+00:00",
        "lang": "en",
        "status": "raw",
    }
    art.update(over)
    return art


@pytest.fixture
def sample_articles():
    return [
        make_article(id="d1", domain="dev_stack", title="Rust 2.0 released", status="selected", score=9.0),
        make_article(id="s1", domain="security", title="CVE-2026-1 critical patch", status="selected", score=8.0),
        make_article(id="a1", domain="ai", title="OpenAI announces frontier model", status="selected", score=9.5),
        make_article(id="a2", domain="ai", title="EU AI Act enforcement update", status="selected", score=8.5),
        make_article(id="b1", domain="business_market", title="Market shift in cloud", status="selected", score=7.0),
    ]


@pytest.fixture
def write_news(tmp_path, monkeypatch):
    """Write a news.json into tmp and point the scripts at it via NEWS_JSON_PATH."""
    import json

    def _write(items, generated_at="2026-06-15T00:00:00+00:00"):
        path = tmp_path / "news.json"
        path.write_text(
            json.dumps({"generated_at": generated_at, "count": len(items),
                        "synthesis": None, "items": items}, ensure_ascii=False),
            encoding="utf-8",
        )
        monkeypatch.setenv("NEWS_JSON_PATH", str(path))
        return path

    return _write
