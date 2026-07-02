"""forecast_news.py — pure logic + a faked model pass. No network, no tokens."""
import json
from types import SimpleNamespace

import forecast_news as fc


# ─── load_ledger / filter_ledger ─────────────────────────────────────────────
def test_load_ledger_skips_blank_and_bad(tmp_path):
    p = tmp_path / "l.jsonl"
    p.write_text(
        '{"date":"2026-05-01","domain":"ai","signal":"a"}\n'
        "\n"
        "not json\n"
        '{"date":"2026-05-02","domain":"ai","signal":"b"}\n',
        encoding="utf-8",
    )
    rows = fc.load_ledger(p)
    assert [r["signal"] for r in rows] == ["a", "b"]


def test_load_ledger_missing_file(tmp_path):
    assert fc.load_ledger(tmp_path / "nope.jsonl") == []


def test_filter_by_domain_and_since():
    rows = [
        {"date": "2026-01-15", "domain": "ai", "signal": "x"},
        {"date": "2026-05-15", "domain": "ai", "signal": "y"},
        {"date": "2026-05-15", "domain": "security", "signal": "z"},
    ]
    assert {r["signal"] for r in fc.filter_ledger(rows, "ai", None)} == {"x", "y"}
    assert {r["signal"] for r in fc.filter_ledger(rows, None, "2026-05")} == {"y", "z"}
    assert {r["signal"] for r in fc.filter_ledger(rows, "ai", "2026-05")} == {"y"}


# ─── build_digest ────────────────────────────────────────────────────────────
def test_build_digest_groups_and_counts():
    rows = [
        {"date": "2026-04-01", "domain": "ai", "signal": "alpha", "score": 8.0},
        {"date": "2026-05-01", "domain": "ai", "signal": "beta",  "score": 6.0},
        {"date": "2026-05-02", "domain": "security", "signal": "gamma", "score": 7.0},
    ]
    d = fc.build_digest(rows)
    assert "## AI" in d and "## SECURITY" in d
    assert "2026-04:1" in d and "2026-05:1" in d   # monthly volume
    assert "alpha" in d and "gamma" in d


# ─── extract_text ────────────────────────────────────────────────────────────
def test_extract_text_skips_non_text_blocks():
    resp = SimpleNamespace(content=[
        SimpleNamespace(type="thinking", thinking="hmm"),
        SimpleNamespace(type="text", text="hello "),
        SimpleNamespace(type="server_tool_use", name="web_search"),
        SimpleNamespace(type="text", text="world"),
    ])
    assert fc.extract_text(resp) == "hello world"


# ─── parse_json ──────────────────────────────────────────────────────────────
def test_parse_json_plain():
    assert fc.parse_json('{"a": 1}') == {"a": 1}


def test_parse_json_strips_fence():
    assert fc.parse_json('```json\n{"a": 1}\n```') == {"a": 1}


def test_parse_json_bad_returns_none():
    assert fc.parse_json("not json") is None
    assert fc.parse_json("") is None


# ─── build_forecast_card ─────────────────────────────────────────────────────
def test_build_forecast_card_shape():
    memo = {
        "title_fr": "FR titre", "title_en": "EN title",
        "content_fr": "deux mots ici", "content_en": "three words here now",
    }
    projections = [{"vector": "V", "projections": [{"horizon": "3mo"}]}]
    card = fc.build_forecast_card(
        "forecast_2026-06-22", "2026-01-01", "2026-06-22",
        memo, projections, source_count=42, now_iso="2026-06-22T00:00:00+00:00",
    )
    assert card["type"] == "synthesis"
    assert card["track"] == "forecast"
    assert card["title_fr"] == "FR titre"
    assert card["vectors"] == projections
    assert card["source_count"] == 42
    assert card["word_count"] == {"fr": 3, "en": 4}


# ─── extract_vectors (faked client) ──────────────────────────────────────────
class _FakeMessages:
    def __init__(self, text):
        self._text = text

    def create(self, **kwargs):
        return SimpleNamespace(
            stop_reason="end_turn",
            content=[SimpleNamespace(type="text", text=self._text)],
        )


class _FakeClient:
    def __init__(self, text):
        self.messages = _FakeMessages(text)


def test_extract_vectors_parses_model_json():
    payload = json.dumps({"vectors": [
        {"name": "Edge WASM", "domain": "dev_stack", "whats_moving": "x",
         "velocity": "accelerating", "evidence": ["a"]},
    ]})
    client = _FakeClient(payload)
    vectors = fc.extract_vectors(client, "digest", [])
    assert len(vectors) == 1
    assert vectors[0]["name"] == "Edge WASM"


def test_extract_vectors_bad_json_returns_empty(monkeypatch):
    monkeypatch.setattr(fc.time, "sleep", lambda _: None)  # skip retry backoff in tests
    assert fc.extract_vectors(_FakeClient("garbage"), "digest", []) == []


# ─── call_and_parse_json retry ───────────────────────────────────────────────
class _FlakyMessages:
    """Returns malformed JSON twice, then valid JSON — proves the retry recovers."""

    def __init__(self, bad_text, good_text, fail_times):
        self._bad, self._good, self._fail_times = bad_text, good_text, fail_times
        self.calls = 0

    def create(self, **kwargs):
        self.calls += 1
        text = self._bad if self.calls <= self._fail_times else self._good
        return SimpleNamespace(
            stop_reason="end_turn",
            content=[SimpleNamespace(type="text", text=text)],
        )


class _FlakyClient:
    def __init__(self, bad_text, good_text, fail_times):
        self.messages = _FlakyMessages(bad_text, good_text, fail_times)


def test_call_and_parse_json_retries_then_succeeds(monkeypatch):
    monkeypatch.setattr(fc.time, "sleep", lambda _: None)
    client = _FlakyClient('{"bad": "json"', json.dumps({"ok": True}), fail_times=2)
    result = fc.call_and_parse_json(client, "test", "prompt", max_tokens=100)
    assert result == {"ok": True}
    assert client.messages.calls == 3


def test_call_and_parse_json_gives_up_after_attempts(monkeypatch):
    monkeypatch.setattr(fc.time, "sleep", lambda _: None)
    client = _FlakyClient("still bad", "irrelevant", fail_times=99)
    result = fc.call_and_parse_json(client, "test", "prompt", max_tokens=100, attempts=3)
    assert result is None
    assert client.messages.calls == 3
