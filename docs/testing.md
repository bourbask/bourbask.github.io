# Pipeline tests — no AI tokens

> Goal: verify the tech-watch jobs work **without ever calling the Anthropic API** or touching the network. Zero tokens spent, deterministic and offline.

The suite lives in `scripts/tests/` (pytest). `make test` runs it.

---

## Principle

Cost (tokens) and flakiness come from two boundaries: the **Anthropic API** and the **network** (RSS/HTTP). The tests neutralize both.

### 1. Dependency-injected Anthropic client → faked

Every model call goes through a client passed as a parameter:
`score_domain(client, …)`, `generate_synthesis(client, …)`, `generate_ai_synthesis(client, …)`,
`proofread_french(client, …)`, `validate_images(client, …)`, `vet_illustration(client, …)`.

Tests pass a `FakeAnthropicClient` (in `conftest.py`) whose `.messages.create()` returns canned text
(score JSON, synthesis JSON, `KEEP`/`REMOVE` lines…). No network call, no tokens, **no
`ANTHROPIC_API_KEY` needed**.

```python
client = fake_client(default=json.dumps([{"id": "x", "score": 8.0, "reason": "ok"}]))
out = sa.score_domain(client, "ai", articles)   # never touches the network
```

The fake also supports `raise_exc=` (to test fallback paths) and `responder=` (a reply computed from
the received prompt, useful when a `main()` chains several calls).

### 2. Network blocked by default

An `autouse` `no_network` fixture (in `conftest.py`) replaces `requests.get` and `feedparser.parse`
with a function that **raises** — unless a test installs its own stub on top. Any accidental network
leak therefore fails loudly instead of hitting the internet (and costing time/tokens).

### 3. Output redirected

Each script reads its output path from `NEWS_JSON_PATH` (else defaults to `public/news.json`). The
`write_news` fixture writes a test `news.json` into a temp directory and points the script at it — so
`main()` integration tests never overwrite the real file.

---

## Coverage

| Module | Verifies |
|--------|----------|
| `test_fetch_news.py` | `classify`, `item_id`, `is_safe_url`, NOISE filter, retention, `fetch_rss`/`fetch_hn` (stubbed network), `ai` domain wired in, `main()` (schema) |
| `test_score_articles.py` | `select_global_top5` (excludes `ai`, arch cap), `select_ai_top`, `score_domain` (parse/sort + fallback), `main()` (statuses, AI selected separately) |
| `test_synthesize_news.py` | dates (`parse_week`, `ai_window`), `cap_articles`, `--track` routing, `generate_ai_synthesis` (EN), `validate_images`, `vet_illustration`, `is_generic_image`, `main()` general + AI |
| `test_repair_synthesis_links.py` | `best_match` (single host, multi-host, threshold, homepage), `repair_links` (preserves anchor, ignores images) |

---

## CI

`.github/workflows/tests.yml` runs on push / pull request touching `scripts/**`.
It installs `scripts/requirements-dev.txt` and runs `pytest scripts/tests -v`.

**No `ANTHROPIC_API_KEY` is provided to the job** → structural proof that no token can be spent.
That is the guarantee we want: the tests validate pipeline logic without ever calling the AI.

---

## Run locally

```bash
make test
# or
python -m pytest scripts/tests -v
```

No environment variable required. If a test fails with `NetworkBlocked`, some code path is attempting
a real network call that isn't stubbed — fix it, don't work around it.
