#!/usr/bin/env python3
"""
Forecast generator — predictive analysis, not a recap.

Where synthesize_news.py answers "what happened this week", this answers
"where is this going, and what should I do about it". It reads the long-term
trend ledger (data/trend_ledger.jsonl — see extract_ledger.py), clusters the
accumulated signals into trend vectors, grounds each vector in a real historical
analogue via web search, projects short/medium/long-term scenarios with explicit
confidence + leading indicators + falsifiers, and ends with a decision memo
addressed to the reader.

Four passes (each a separate model call so the model can reason):
  A. EXTRACT VECTORS  — ledger digest → 3-5 trend vectors (what's moving, evidence, velocity)
  B. GROUND + PROJECT — per vector, web_search a historical analogue, then project
                        (this is the pass that uses the web_search server tool)
  C. DECISION MEMO    — synthesize all projections into a bilingual FR+EN memo

Model: claude-opus-4-8 (the analysis is the whole point — worth the Opus tier),
adaptive thinking, effort high. Cadence is monthly, so cost stays bounded.

Usage:
  python scripts/forecast_news.py                 # forecast from the full ledger
  python scripts/forecast_news.py --domain ai     # restrict to one domain
  python scripts/forecast_news.py --since 2026-01 # only signals on/after this month
  python scripts/forecast_news.py --force          # overwrite today's forecast
  python scripts/forecast_news.py --dry-run        # print, don't write news.json
"""
import argparse
import json
import os
import re
import sys
import time
from collections import Counter, defaultdict
from datetime import datetime, timezone
from pathlib import Path

REPO_ROOT   = Path(__file__).resolve().parent.parent
LEDGER_PATH = REPO_ROOT / "data" / "trend_ledger.jsonl"

MODEL          = "claude-opus-4-8"
MAX_VECTORS    = 5
MIN_SIGNALS    = 12   # below this the ledger is too thin to forecast meaningfully


def _load_dotenv() -> None:
    env = REPO_ROOT / ".env"
    if not env.exists():
        return
    for line in env.read_text(encoding="utf-8").splitlines():
        line = line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        k, _, v = line.partition("=")
        k, v = k.strip(), v.strip().strip('"').strip("'")
        if k and k not in os.environ:
            os.environ[k] = v

_load_dotenv()

try:
    import anthropic
except ImportError:
    print("Missing deps: pip install anthropic", file=sys.stderr)
    sys.exit(1)


# ─── pure logic (unit-tested, no network) ────────────────────────────────────
def load_ledger(path: Path) -> list[dict]:
    if not path.exists():
        return []
    rows = []
    for line in path.read_text(encoding="utf-8").splitlines():
        line = line.strip()
        if not line:
            continue
        try:
            rows.append(json.loads(line))
        except json.JSONDecodeError:
            continue
    return rows


def filter_ledger(rows: list[dict], domain: str | None, since: str | None) -> list[dict]:
    out = rows
    if domain:
        out = [r for r in out if r.get("domain") == domain]
    if since:  # "YYYY-MM" → keep signals on/after that month
        out = [r for r in out if (r.get("date", "")[:7] >= since)]
    return out


def build_digest(rows: list[dict], max_per_domain: int = 35) -> str:
    """
    Compact the ledger into a per-domain monthly timeline the model can reason
    over without blowing the token budget: monthly signal counts (the velocity
    signal) plus the highest-scored signals per domain (the substance).
    """
    by_domain: dict[str, list[dict]] = defaultdict(list)
    for r in rows:
        by_domain[r.get("domain", "?")].append(r)

    blocks = []
    for domain in sorted(by_domain):
        sigs = by_domain[domain]
        monthly = Counter(s.get("date", "")[:7] for s in sigs if s.get("date"))
        volume = " ".join(f"{m}:{monthly[m]}" for m in sorted(monthly))
        top = sorted(sigs, key=lambda s: s.get("score") or 0.0, reverse=True)[:max_per_domain]
        top.sort(key=lambda s: s.get("date", ""))
        lines = [
            f"  {s.get('date','?')} [{(s.get('score') or 0):.0f}] {s.get('signal','')[:90]}"
            for s in top
        ]
        blocks.append(
            f"## {domain.upper().replace('_', ' ')}  ({len(sigs)} signals)\n"
            f"monthly volume: {volume}\n" + "\n".join(lines)
        )
    return "\n\n".join(blocks)


def extract_text(response) -> str:
    """Concatenate text-type content blocks, skipping thinking / tool blocks."""
    parts = []
    for block in response.content:
        if getattr(block, "type", None) == "text":
            parts.append(block.text)
    return "".join(parts).strip()


def parse_json(text: str) -> dict | list | None:
    """Strip any markdown fence and json.loads — same tolerance as synthesize_news.py."""
    if not text:
        return None
    text = re.sub(r"^```(?:json)?\s*\n?", "", text.strip())
    text = re.sub(r"\n?```\s*$", "", text)
    try:
        return json.loads(text)
    except json.JSONDecodeError:
        return None


def build_forecast_card(
    forecast_id: str,
    period_start: str,
    period_end: str,
    memo: dict,
    projections: list[dict],
    source_count: int,
    now_iso: str,
) -> dict:
    """Assemble the news.json card (track='forecast'), mirroring the synthesis shape."""
    return {
        "id":           forecast_id,
        "type":         "synthesis",
        "track":        "forecast",
        "title_fr":     memo.get("title_fr", "Prospective tech"),
        "title_en":     memo.get("title_en", "Tech Forecast"),
        "period_start": period_start,
        "period_end":   period_end,
        "published_at": now_iso,
        "content_fr":   memo.get("content_fr", ""),
        "content_en":   memo.get("content_en", ""),
        "security_actions": [],
        "architecture_visual": None,
        "illustration": None,
        "vectors":      projections,   # the structured forecast behind the prose
        "word_count":   {
            "fr": len((memo.get("content_fr") or "").split()),
            "en": len((memo.get("content_en") or "").split()),
        },
        "source_count": source_count,
        "sources":      [],
    }


# ─── model passes (network) ──────────────────────────────────────────────────
def _call(client, prompt: str, max_tokens: int, web_search: bool = False) -> str:
    """One Opus call (adaptive thinking, effort high). Optionally with web_search.

    web_search is a server-side tool: results come back in the same response. If
    the server tool loop hits its iteration cap the response stops with
    `pause_turn` — re-send to resume.
    """
    kwargs = dict(
        model=MODEL,
        max_tokens=max_tokens,
        thinking={"type": "adaptive"},
        output_config={"effort": "high"},
    )
    if web_search:
        kwargs["tools"] = [{
            "type": "web_search_20260209",
            "name": "web_search",
            "max_uses": 6,
        }]

    messages = [{"role": "user", "content": prompt}]
    for _ in range(4):  # bounded pause_turn resumes
        resp = client.messages.create(messages=messages, **kwargs)
        if resp.stop_reason == "pause_turn":
            messages = [
                {"role": "user", "content": prompt},
                {"role": "assistant", "content": resp.content},
            ]
            continue
        return extract_text(resp)
    return extract_text(resp)


def call_and_parse_json(
    client, log_prefix: str, prompt: str, max_tokens: int,
    web_search: bool = False, attempts: int = 3,
) -> dict | list | None:
    """
    Run _call() and parse_json() together, retrying the whole (expensive) call
    on JSONDecodeError — same rationale as synthesize_news.py's call_claude_json:
    the model occasionally emits malformed JSON, a transient glitch worth a retry
    rather than losing the whole forecast pass.
    """
    last_err: Exception | None = None
    for attempt in range(1, attempts + 1):
        text = _call(client, prompt, max_tokens, web_search=web_search)
        try:
            text_stripped = re.sub(r"^```(?:json)?\s*\n?", "", text.strip())
            text_stripped = re.sub(r"\n?```\s*$", "", text_stripped)
            return json.loads(text_stripped)
        except json.JSONDecodeError as e:
            last_err = e
            print(f"[{log_prefix}] JSON parse failed (attempt {attempt}/{attempts}): {e}", file=sys.stderr)
            if attempt < attempts:
                time.sleep(2 * attempt)
    print(f"[{log_prefix}] generation failed after {attempts} attempts: {last_err}", file=sys.stderr)
    return None


def extract_vectors(client, digest: str, prev_titles: list[str]) -> list[dict]:
    prev = ""
    if prev_titles:
        prev = ("\nPREVIOUS FORECASTS (pick fresh or genuinely evolved vectors, "
                "don't just repeat):\n" + "\n".join(f"  - {t}" for t in prev_titles))
    prompt = f"""You are a technology forecasting analyst. Below is a long-term ledger of
qualified tech signals (the ones that cleared a quality bar over the past months),
grouped by domain, with monthly volume counts and the highest-scored signals.

Your job in THIS step only: cluster the signals into {MAX_VECTORS} or fewer TREND VECTORS.
A vector is a direction something is moving — not a topic, a *trajectory*. Use the
monthly volume to judge velocity (accelerating, steady, fading).

LEDGER:
{digest}
{prev}

Output PURE JSON, nothing else:
{{
  "vectors": [
    {{
      "name": "short punchy vector name",
      "domain": "primary domain",
      "whats_moving": "one sentence — what is actually changing",
      "velocity": "accelerating | steady | fading",
      "evidence": ["signal essence + its date", "..."]
    }}
  ]
}}"""
    data = call_and_parse_json(client, "forecast:vectors", prompt, max_tokens=4000)
    return (data or {}).get("vectors", []) if isinstance(data, dict) else []


def ground_and_project(client, vector: dict) -> dict | None:
    prompt = f"""You are a forecasting analyst. You have ONE trend vector to analyze.

VECTOR: {vector.get('name')}
WHAT'S MOVING: {vector.get('whats_moving')}
VELOCITY: {vector.get('velocity')}
PRESENT EVIDENCE (from a tracked signal ledger):
{chr(10).join('  - ' + e for e in vector.get('evidence', []))}

STEP 1 — Use web_search to find the closest HISTORICAL ANALOGUE: a prior technology
cycle, regulation wave, or adoption curve that rhymes with this vector. Find what that
precedent actually did NEXT (its outcome), with real dates and sources.

STEP 2 — Project this vector forward. Every projection MUST carry a confidence level,
a leading indicator the reader can check next month, and a falsifier (what would prove
it wrong). No hedging without a number. No "time will tell".

Output PURE JSON, nothing else (no markdown fence):
{{
  "vector": "{vector.get('name')}",
  "historical_analogue": "the precedent + why it rhymes",
  "precedent_outcome": "what that precedent did next",
  "sources": ["url", "..."],
  "projections": [
    {{"horizon": "3mo",  "confidence": "high|medium|low", "claim": "...", "leading_indicator": "...", "falsifier": "..."}},
    {{"horizon": "12mo", "confidence": "high|medium|low", "claim": "...", "leading_indicator": "...", "falsifier": "..."}},
    {{"horizon": "3yr",  "confidence": "high|medium|low", "claim": "...", "leading_indicator": "...", "falsifier": "..."}}
  ],
  "so_what": "concrete action/bet for a senior dev (Rust/WASM/security/Linux)"
}}"""
    data = call_and_parse_json(client, "forecast:ground", prompt, max_tokens=6000, web_search=True)
    return data if isinstance(data, dict) and data.get("projections") else None


def decision_memo(client, projections: list[dict], period_start: str, period_end: str) -> dict | None:
    prompt = f"""You write a technology FORECAST memo — predictive, not a recap. Two versions:
French and English. The reader is a senior web developer (Rust, WebAssembly, web security,
Linux), opinionated, allergic to hype. The point is that they can ACT on this.

Below are analyzed trend vectors — each with a historical analogue, projections (with
confidence + leading indicators + falsifiers), and a "so what". Period: {period_start} → {period_end}.

VECTORS (JSON):
{json.dumps(projections, ensure_ascii=False, indent=2)}

Write a memo that, for each vector worth keeping: states where it sits now, the historical
rhyme, the projection with its timeframe + confidence, and the decision it implies. Then a
short closing: bet on X / hedge Y / ignore Z. Keep the falsifiers visible — next month we
score these calls.

VOICE: human, sharp, varied rhythm. Banned: "En résumé", "Force est de constater", "delve",
"In a world where", "il est intéressant de noter", stacked mechanical transitions, false
balance. Markdown GFM, no "# Title" line (it's shown separately). The FR and EN titles must
be DIFFERENT angles, not a translation.

Output PURE JSON, nothing else (no markdown fence):
{{
  "title_fr": "titre prospectif accrocheur (max 80 chars)",
  "title_en": "punchy forecast title (max 80 chars, different angle)",
  "content_fr": "[mémo FR — commence par le texte, PAS par # Titre]",
  "content_en": "[English memo — starts with text, NOT # Title]"
}}"""
    return call_and_parse_json(client, "forecast:memo", prompt, max_tokens=12000)  # type: ignore[return-value]


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--domain", help="Restrict to one domain (e.g. ai)")
    ap.add_argument("--since", help="Only signals on/after this month, YYYY-MM")
    ap.add_argument("--force", action="store_true", help="Overwrite today's forecast")
    ap.add_argument("--dry-run", action="store_true", help="Print, don't write news.json")
    args = ap.parse_args()

    api_key = os.environ.get("ANTHROPIC_API_KEY")
    if not api_key:
        print("[forecast] ANTHROPIC_API_KEY not set", file=sys.stderr)
        sys.exit(1)

    rows = filter_ledger(load_ledger(LEDGER_PATH), args.domain, args.since)
    if len(rows) < MIN_SIGNALS:
        print(f"[forecast] Ledger too thin: {len(rows)} signals (need ≥ {MIN_SIGNALS}). "
              f"Run scoring for a while, or scripts/extract_ledger.py to backfill.",
              file=sys.stderr)
        sys.exit(0)

    dates = sorted(r["date"] for r in rows if r.get("date"))
    period_start, period_end = dates[0], dates[-1]
    now = datetime.now(timezone.utc)
    forecast_id = f"forecast_{now.strftime('%Y-%m-%d')}"
    if args.domain:
        forecast_id += f"_{args.domain}"

    out_path = Path(os.environ.get("NEWS_JSON_PATH") or (REPO_ROOT / "public" / "news.json"))
    data = json.loads(out_path.read_text(encoding="utf-8")) if out_path.exists() else {"items": []}

    if not args.dry_run and not args.force and any(i.get("id") == forecast_id for i in data["items"]):
        print(f"[forecast] {forecast_id} already exists. Use --force to overwrite.")
        sys.exit(0)

    prev_titles = [
        i.get("title_en", "")
        for i in data.get("items", [])
        if i.get("type") == "synthesis" and i.get("track") == "forecast"
    ][:3]

    digest = build_digest(rows)
    print(f"[forecast] {len(rows)} signals, {period_start} → {period_end}", file=sys.stderr)

    client = anthropic.Anthropic(api_key=api_key)

    print("[forecast] Pass A — extracting trend vectors…", file=sys.stderr)
    vectors = extract_vectors(client, digest, prev_titles)
    if not vectors:
        print("[forecast] No vectors extracted — aborting.", file=sys.stderr)
        sys.exit(1)
    print(f"[forecast]   {len(vectors)} vectors: {', '.join(v.get('name','?') for v in vectors)}",
          file=sys.stderr)

    print("[forecast] Pass B — grounding in history + projecting (web_search)…", file=sys.stderr)
    projections = []
    for v in vectors:
        proj = ground_and_project(client, v)
        if proj:
            projections.append(proj)
            print(f"[forecast]   ✓ {v.get('name','?')}", file=sys.stderr)
        else:
            print(f"[forecast]   ✗ {v.get('name','?')} (no usable projection)", file=sys.stderr)
    if not projections:
        print("[forecast] No projections produced — aborting.", file=sys.stderr)
        sys.exit(1)

    print("[forecast] Pass C — writing decision memo…", file=sys.stderr)
    memo = decision_memo(client, projections, period_start, period_end)
    if not memo:
        print("[forecast] Memo generation failed — aborting.", file=sys.stderr)
        sys.exit(1)

    card = build_forecast_card(
        forecast_id, period_start, period_end, memo, projections,
        source_count=len(rows), now_iso=now.isoformat(),
    )

    if args.dry_run:
        print(f"\n[forecast] --dry-run — {forecast_id}", file=sys.stderr)
        print(f"  FR: {card['title_fr']}\n  EN: {card['title_en']}", file=sys.stderr)
        print(f"  words FR:{card['word_count']['fr']} EN:{card['word_count']['en']}, "
              f"{len(projections)} vectors", file=sys.stderr)
        return

    data["items"] = [card] + [i for i in data["items"] if i.get("id") != forecast_id]
    data["generated_at"] = now.isoformat()
    data["count"] = len(data["items"])
    out_path.write_text(json.dumps(data, ensure_ascii=False, indent=2), encoding="utf-8")
    print(f"[forecast] Done: {forecast_id} — {len(projections)} vectors, "
          f"FR:{card['word_count']['fr']}w EN:{card['word_count']['en']}w", file=sys.stderr)


if __name__ == "__main__":
    main()
