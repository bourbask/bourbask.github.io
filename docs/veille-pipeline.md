# Tech Watch Pipeline — full documentation

> Last updated: 2026-06-18 (push notification step, NOTIFY_SECRET)

---

## Overview

A **fully automated** pipeline, orchestrated by a single workflow (`news-pipeline.yml`):

1. **Fetch** (daily, no AI) — collects from ~45 sources across 6 domains.
2. **Score** (daily, Claude Haiku) — per-domain competition → general top 5 **plus** a dedicated AI top.
3. **General synthesis** (Monday, Claude Sonnet) — bilingual FR + EN editorial article. **AI is excluded.**
4. **AI brief** (Monday **and** Thursday, Claude Sonnet) — dedicated AI synthesis, **English only**, short, token-lean.

AI is treated as its own domain (`ai`), with its own tab on `/veille` and its own synthesis, because access to frontier models has become a geopolitically critical topic that warrants finer tracking.

---

## Domains & sources

| Domain | Sources (excerpt) |
|--------|-------------------|
| `dev_stack` (11) | Rust Blog, This Week in Rust, Go Blog, Mozilla Hacks, WebKit, GitHub Eng, LWN, Lobste.rs, LinuxFr, ArXiv CS.PL |
| `ai` (16) | OpenAI, Google DeepMind, Hugging Face, Microsoft Research, ArXiv CS.AI/LG/CL/CY, Google Research, BAIR, EU Digital Strategy, NIST, Import AI, The Gradient, MIT Tech Review AI, Ars Technica AI |
| `security` (8) | CISA, NIST CSRC, ENISA, OpenSSF, Krebs, Schneier, PortSwigger, SANS ISC |
| `health_science` (4) | ArXiv CS.HC, ArXiv q-bio, eLife, PLOS ONE |
| `business_market` (7) | The Register, InfoQ, CNCF, W3C, IETF, IEEE Spectrum, ACM Tech News + HackerNews (≥150) |
| `architecture` (5) | Dezeen Sustainable, Low-tech Magazine, ArchDaily, TreeHugger, Resilient Design |

The `ai` domain spans 4 axes: **official labs**, **academic research**, **regulatory/legal**, **AI analysis/safety**.
Formerly `ai_emerging`; IEEE/ACM moved to `business_market`, ArXiv CS.PL moved to `dev_stack`.

> Feed URLs are validated by `scripts/check_feeds.py` (network, no AI) — workflow `feeds-smoke.yml`.

---

## Schedule (single cron)

```
UTC time   │ Mon  Tue  Wed  Thu  Fri  Sat  Sun
───────────┼────────────────────────────────────
06:00      │  ●    ●    ●    ●    ●    ●    ●     news-pipeline.yml
           │                                      ├─ fetch_news.py         (always, no AI)
           │                                      ├─ score_articles.py     (always, Haiku)
           │  G                                   ├─ synth --track general (Monday)
           │  A              A                    └─ synth --track ai      (Monday + Thursday)
```

`workflow_dispatch` accepts: `force_synthesis`, `force_ai_synthesis`, `score_date`, `score_week`.

---

## Step 2 — scoring & selection

`score_articles.py` scores each domain with Claude Haiku (0–10), then makes **two** selections:

- **General top 5** (`select_global_top5`) — across all domains **except `ai`**, max 1 `architecture` article.
- **AI top** (`select_ai_top`, default 6) — the best articles of the `ai` domain.

Both sets become `status="selected"`; the rest become `archived`.

### Trend ledger (long-term memory)
After writing `news.json`, the scorer appends the run's **qualified** signals
(`status=selected` **or** `score ≥ 6`) to `data/trend_ledger.jsonl` — one compact,
distilled line each (`date, domain, signal, score, status, …`). This file is
**append-only and permanent**: it survives `news.json` retention so the forecast
step can read a real *trajectory* instead of a one-week snapshot. The append is
idempotent (dedup by signal id) and best-effort (a ledger error never fails scoring).

Backfill from history: `scripts/extract_ledger.py` mines every past git revision of
`news.json` + synthesis sources (deterministic, **zero tokens**) — bootstraps the
ledger before live accumulation takes over.

### Per-domain criteria (excerpt)
- `ai` — strategic/regulatory signal (lab announcements, model access & export controls, EU AI Act/NIST) **40%** · reproducible scientific substance **40%** · novelty vs recycled hype **20%**.
- `dev_stack`, `security`, `health_science`, `business_market`, `architecture` — see `DOMAIN_CRITERIA` in `score_articles.py`.

---

## Step 3 — syntheses

### Article cap (anti-truncation)
Scoring keeps ~5 `selected`/day, so ~35 over 7 days. Feeding them all into one prompt makes the
output **truncate → invalid JSON → failure**. Hence a top-N cap by score:
`MAX_SYNTHESIS_ARTICLES = 8` (general, max 3/domain) · `MAX_AI_ARTICLES = 6` (AI).

### General synthesis (`--track general`, Monday)
- Candidates: `selected` from the past 7 days, **`domain != "ai"`**.
- Claude Sonnet → bilingual FR + EN article, human editorial voice (see "Voice" below).
- Security present → mandatory "Immediate actions" section. Architecture → Wikipedia bio/visual.
- French proofread (Haiku) + inline image validation (Haiku).
- ID: `synthesis_YYYY_WNN`. Field `track: "general"`.

### AI brief (`--track ai`, Monday + Thursday) — **token-lean**
- Candidates: `selected`, **`domain == "ai"`**, over a **rolling window** since the last AI brief's `period_end` (else `now-7d`).
- **English only, short (~500-900 words), no illustration** → **a single Sonnet call** (reduced max_tokens), **zero Haiku**, no image fetch.
- Structure: what happened → where it sits in the state of the art → short/medium/long-term implications → developer actions.
- Date-based ID: `synthesis_ai_YYYY-MM-DD`. Field `track: "ai"`. `content_fr` empty (the frontend renders EN).

### Forecast (`forecast_news.py`, `--track forecast`) — predictive, not a recap
Where the syntheses recap *what happened*, the forecast answers *where it's going and what to
do*. It reads the **trend ledger** (not the 7-day window) and runs a 4-pass agent on
**Claude Opus 4.8** (adaptive thinking, effort high) — cadence is monthly, so cost stays bounded:

1. **Extract vectors** — ledger digest → ≤5 trend vectors (what's moving, evidence, velocity).
2. **Ground + project** — per vector, the **`web_search` server tool** finds a real historical
   analogue, then projects short/medium/long-term scenarios. Every projection carries a
   **confidence**, a **leading indicator** (checkable next month) and a **falsifier**.
3. **Decision memo** — synthesizes the projections into a bilingual FR+EN memo: bet on X / hedge
   Y / ignore Z. Falsifiers stay visible so past calls can be scored later.

Card: `track: "forecast"`, ID `forecast_YYYY-MM-DD` (`_<domain>` suffix when `--domain` is used).
The structured projections are kept on the card under `vectors[]` alongside the prose.
Needs ≥ 12 ledger signals; before the ledger has depth, `web_search` carries the cold start.

### Editorial voice
Senior developer (Rust/WASM/security/Linux), opinionated, human and lively register.
Prompts carry a **banned anti-pattern list** ("In summary", "delve into", stacked mechanical
transitions, false balance) and **forbid exposing the prompt** (no meta summary-conclusion). The
structure is an intention, not a template.

### Source-link integrity
The model never types a source article's URL (it mangles them → 404s). Instead:
- the prompt references each source by a `srcref:<id>` token;
- `resolve_source_links()` swaps the token for the **exact feed URL** after generation.

Repairing existing content: `scripts/repair_synthesis_links.py` (deterministic, no AI) matches each
broken link against the synthesis `sources[]` (same host → best slug + title similarity, confidence
threshold, homepages skipped). An unresolvable link is left as-is rather than pointed at the wrong
topic.

### Illustration
The hero visual (`illustration`) goes through a generic-image denylist (`find_images` → envelope,
logo, icon…) **plus** a Haiku relevance check (`vet_illustration`, general track only). The AI brief
has no illustration.

---

## Step 5 — push notifications

After commit, the pipeline notifies PWA subscribers via Web Push:

```

Commit (step 4)
    │
    ├──→ notify_push.mjs
    │      ├── Count recent selected articles (< 3 days old)
    │      │   └── 0 articles → exit (no notification)
    │      ├── Fetch subs from Worker: GET /sub/subs (X-Notify-Secret header)
    │      ├── webpush.sendNotification() to each subscriber
    │      └── POST /sub/unsubscribe (X-Notify-Secret) for invalid endpoints
    │
    └──→ done
```

- VAPID keys: `VAPID_PUBLIC_KEY` / `VAPID_PRIVATE_KEY` (GitHub secrets).
- `NOTIFY_SECRET` authenticates the notify script against the Worker (header, not query param).
- Invalid subscriptions (HTTP 404/410) are cleaned up automatically.
- Only `status=selected` articles less than 3 days old trigger a notification.
- Client-side registration: `public/js/push-notifications.js` (registers SW + subscribes).
- Worker endpoints: `/sub/subscribe` (CORS, rate-limited 20/h/IP), `/sub/subs` (auth header), `/sub/unsubscribe` (auth header).

---

## Data structure: `public/news.json`

```
items[]
├── [type="article"]
│   ├── id, type, title, url, source
│   ├── domain        dev_stack | ai | security | health_science | business_market | architecture
│   ├── categories[]  [urgent | good_news | future_watch | stack_alt | general]
│   ├── published_at, fetched_at, lang
│   ├── status        raw | selected | archived
│   ├── score, score_reason          (set by score_articles.py)
│   └── synthesis_id  "synthesis_YYYY_WNN" | "synthesis_ai_YYYY-MM-DD"
│
└── [type="synthesis"]
    ├── id            "synthesis_YYYY_WNN" (general) | "synthesis_ai_YYYY-MM-DD" (AI)
    ├── track         "general" | "ai"
    ├── title_fr, title_en, content_fr, content_en   (AI: content_fr empty, title mirrored)
    ├── security_actions[], architecture_visual, illustration
    ├── period_start, period_end, published_at, word_count, source_count
    └── sources[]     {id, title, url, source, domain, lang, published_at, score}
```

**Retention by status:** `synthesis` indefinite · `selected` 21 d · `archived` 4 d · `raw` 2 d · legacy 14 d.

---

## Local development

```bash
make fetch        # RSS + HN (no AI)
make score        # daily scoring (Haiku)             — ANTHROPIC_API_KEY required
make score-dry    # scoring without writing
make synth        # general synthesis (--track general)
make synth-ai     # AI brief (--track ai)
make test         # pipeline tests — offline, zero tokens

# Repair links in older syntheses (deterministic, no AI)
python scripts/repair_synthesis_links.py --dry-run

# Targeted scoring (catch-up)
python scripts/score_articles.py --date 2026-06-01 --dry-run
python scripts/score_articles.py --week 2026-W22
```

Tests: see [testing.md](testing.md) — offline pytest suite, faked Anthropic client, **zero token
usage**.

---

## Key files

```
.github/workflows/
├── news-pipeline.yml    Cron 06:00 — fetch + score; general synthesis (Mon), AI brief (Mon+Thu)
├── tests.yml            pytest pipeline (no API key → zero tokens)
├── feeds-smoke.yml      Feed reachability check (no AI)
├── tools-pipeline.yml   Open-source tool discovery + article PR (weekly)
├── deploy.yml           Build & deploy to GitHub Pages
└── quality.yml          Post-deploy validation

scripts/
├── fetch_news.py              RSS + HN aggregator + classification (no AI)
├── score_articles.py          Per-domain competition + general/AI selection (Haiku)
├── synthesize_news.py         General synthesis + AI brief (Sonnet), --track {general,ai}
├── repair_synthesis_links.py  Deterministic source-link repair (no AI)
├── check_feeds.py             Feed smoke check (no AI)
├── tests/                     Offline pytest suite (conftest + 4 modules)
└── requirements.txt / requirements-dev.txt

src/components/
└── veille.rs                  /veille page (AI tab, AI-brief badge, synthesis detail)
```
