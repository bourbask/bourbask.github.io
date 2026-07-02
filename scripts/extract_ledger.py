#!/usr/bin/env python3
"""
Trend ledger backfill — reconstruct a long-term signal memory from git history.

The live pipeline forgets fast (selected 21d, archived 4d, raw 2d). Forecasting
needs a *trajectory*, not a snapshot — so we mine every past revision of
public/news.json (each commit froze a snapshot of then-live signals) plus the
syntheses' source lists, dedup by id, keep the best-known record of each signal,
and append the qualified ones to an append-only ledger.

The ledger is DISTILLED HISTORY, not raw data: one compact line per qualified
signal — date, domain, essence (title), score, status. It survives retention so
that after a few months the forecast agent can read a real timeline.

Deterministic. No network, no Anthropic calls, zero tokens.

Usage:
  python scripts/extract_ledger.py                 # backfill from full git history
  python scripts/extract_ledger.py --dry-run       # show what would be written
  python scripts/extract_ledger.py --min-score 7   # stricter qualification
  python scripts/extract_ledger.py --selected-only # only ever-selected signals
  python scripts/extract_ledger.py --all           # no quality filter (noisy)
"""
import argparse
import json
import subprocess
import sys
from pathlib import Path

REPO_ROOT   = Path(__file__).resolve().parent.parent
NEWS_PATH   = "public/news.json"
LEDGER_PATH = REPO_ROOT / "data" / "trend_ledger.jsonl"

# Legacy/alias domains → canonical. ai_emerging was folded into ai (see veille-pipeline.md).
DOMAIN_ALIASES = {"ai_emerging": "ai"}

# status quality rank — higher wins when merging duplicate signals across revisions.
STATUS_RANK = {"selected": 3, "archived": 2, "raw": 1, None: 0, "": 0}

DEFAULT_MIN_SCORE = 6.0  # a signal qualifies if it was ever selected OR scored >= this


def normalize_domain(domain: str) -> str:
    d = (domain or "").strip()
    return DOMAIN_ALIASES.get(d, d)


def valid_date(date_str: str) -> bool:
    """Reject empty and obviously bogus dates (e.g. 0001-01-01 from bad feeds)."""
    return bool(date_str) and date_str[:4].isdigit() and date_str[:4] >= "2020"


def signal_records(news: dict) -> dict:
    """
    Extract one record per signal from a parsed news.json dict.
    Mines both article items and the source[] lists embedded in syntheses
    (syntheses are kept forever, so their sources recover long-pruned signals).
    Keyed by signal id for downstream dedup. Returns {id: record}.
    """
    out: dict[str, dict] = {}
    for item in news.get("items", []):
        if item.get("type", "article") == "article":
            sig_id = item.get("id") or item.get("url")
            if not sig_id:
                continue
            out[sig_id] = {
                "id":         sig_id,
                "date":       (item.get("published_at") or "")[:10],
                "domain":     normalize_domain(item.get("domain", "")),
                "signal":     (item.get("title") or "").strip(),
                "source":     item.get("source", ""),
                "url":        item.get("url", ""),
                "score":      item.get("score"),
                "status":     item.get("status"),
                "categories": item.get("categories", []),
                "lang":       item.get("lang", ""),
                "tier":       "tracked",
            }
        else:
            # synthesis (or other) — recover its source signals
            for s in item.get("sources", []):
                sig_id = s.get("id") or s.get("url")
                if not sig_id:
                    continue
                out[sig_id] = {
                    "id":         sig_id,
                    "date":       (s.get("published_at") or "")[:10],
                    "domain":     normalize_domain(s.get("domain", "")),
                    "signal":     (s.get("title") or "").strip(),
                    "source":     s.get("source", ""),
                    "url":        s.get("url", ""),
                    "score":      s.get("score"),
                    # appearing as a synthesis source means it cleared selection
                    "status":     "selected",
                    "categories": [],
                    "lang":       s.get("lang", ""),
                    "tier":       "tracked",
                }
    return out


def _better(a: dict, b: dict) -> dict:
    """Pick the richer of two records for the same signal id."""
    ra, rb = STATUS_RANK.get(a.get("status"), 0), STATUS_RANK.get(b.get("status"), 0)
    if ra != rb:
        return a if ra > rb else b
    sa, sb = a.get("score") or 0.0, b.get("score") or 0.0
    if sa != sb:
        return a if sa > sb else b
    # tie-break: prefer one with a valid date, then a domain
    if valid_date(a.get("date", "")) != valid_date(b.get("date", "")):
        return a if valid_date(a.get("date", "")) else b
    return a if a.get("domain") else b


def merge_records(*record_maps: dict) -> dict:
    """Merge {id: record} maps, keeping the best-known record per id."""
    merged: dict[str, dict] = {}
    for rmap in record_maps:
        for sig_id, rec in rmap.items():
            merged[sig_id] = _better(merged[sig_id], rec) if sig_id in merged else rec
    return merged


def qualifies(rec: dict, *, selected_only: bool, min_score: float, keep_all: bool) -> bool:
    """A signal earns a ledger line if it cleared the quality bar."""
    if not rec.get("signal") or not rec.get("domain") or not valid_date(rec.get("date", "")):
        return False
    if keep_all:
        return True
    if rec.get("status") == "selected":
        return True
    if selected_only:
        return False
    return (rec.get("score") or 0.0) >= min_score


def to_ledger_line(rec: dict) -> dict:
    """Compact, stable ledger entry — distilled, not the full article."""
    return {
        "date":     rec["date"],
        "domain":   rec["domain"],
        "signal":   rec["signal"],
        "source":   rec.get("source", ""),
        "score":    round(rec["score"], 1) if isinstance(rec.get("score"), (int, float)) else None,
        "status":   rec.get("status"),
        "category": (rec.get("categories") or [None])[0],
        "url":      rec.get("url", ""),
        "id":       rec["id"],
        "tier":     "tracked",
    }


# ─── git driver (impure) ────────────────────────────────────────────────────
def git_revisions(path: str) -> list[str]:
    res = subprocess.run(
        ["git", "log", "--format=%H", "--follow", "--", path],
        cwd=REPO_ROOT, capture_output=True, text=True,
    )
    return [h for h in res.stdout.split() if h]


def news_at_revision(rev: str, path: str) -> dict | None:
    res = subprocess.run(
        ["git", "show", f"{rev}:{path}"],
        cwd=REPO_ROOT, capture_output=True, text=True,
    )
    if res.returncode != 0 or not res.stdout:
        return None
    try:
        return json.loads(res.stdout)
    except json.JSONDecodeError:
        return None


def write_ledger(final_map: dict) -> list[dict]:
    """Sort by (date, domain) and write the ledger atomically. Returns the lines."""
    lines = sorted(final_map.values(), key=lambda r: (r.get("date", ""), r.get("domain", "")))
    LEDGER_PATH.parent.mkdir(parents=True, exist_ok=True)
    with LEDGER_PATH.open("w", encoding="utf-8") as f:
        for l in lines:
            f.write(json.dumps(l, ensure_ascii=False) + "\n")
    return lines


def append_signals(
    news_items: list[dict],
    *,
    selected_only: bool = False,
    min_score: float = DEFAULT_MIN_SCORE,
) -> int:
    """
    Live hook for the daily pipeline: distill freshly-scored items into ledger
    lines and merge them in (idempotent — re-scoring the same day is safe).
    Returns the number of NEW signals added. Zero tokens, no network.
    """
    recs = signal_records({"items": news_items})
    qualified = {
        sig_id: rec for sig_id, rec in recs.items()
        if qualifies(rec, selected_only=selected_only, min_score=min_score, keep_all=False)
    }
    existing = load_existing_ledger()
    before = len(existing)
    final_map = merge_records(
        existing,
        {k: to_ledger_line(v) for k, v in qualified.items()},
    )
    write_ledger(final_map)
    return len(final_map) - before


def load_existing_ledger() -> dict:
    """Existing ledger keyed by id, so re-running is idempotent (no dupes)."""
    if not LEDGER_PATH.exists():
        return {}
    out = {}
    for line in LEDGER_PATH.read_text(encoding="utf-8").splitlines():
        line = line.strip()
        if not line:
            continue
        try:
            rec = json.loads(line)
        except json.JSONDecodeError:
            continue
        if rec.get("id"):
            out[rec["id"]] = rec
    return out


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--dry-run", action="store_true", help="Print stats, don't write")
    ap.add_argument("--min-score", type=float, default=DEFAULT_MIN_SCORE,
                    help=f"Score threshold to qualify a non-selected signal (default {DEFAULT_MIN_SCORE})")
    ap.add_argument("--selected-only", action="store_true",
                    help="Only keep signals that were ever selected")
    ap.add_argument("--all", dest="keep_all", action="store_true",
                    help="No quality filter — keep every dated signal (noisy)")
    args = ap.parse_args()

    # 1. Working copy
    work_path = REPO_ROOT / NEWS_PATH
    maps = []
    if work_path.exists():
        maps.append(signal_records(json.loads(work_path.read_text(encoding="utf-8"))))

    # 2. Every past revision
    revs = git_revisions(NEWS_PATH)
    print(f"[ledger] scanning {len(revs)} git revisions of {NEWS_PATH}…", file=sys.stderr)
    for rev in revs:
        news = news_at_revision(rev, NEWS_PATH)
        if news:
            maps.append(signal_records(news))

    history = merge_records(*maps)
    print(f"[ledger] {len(history)} unique signals across history", file=sys.stderr)

    qualified = {
        sig_id: rec for sig_id, rec in history.items()
        if qualifies(rec, selected_only=args.selected_only,
                     min_score=args.min_score, keep_all=args.keep_all)
    }
    print(f"[ledger] {len(qualified)} qualified (status=selected or score>={args.min_score})",
          file=sys.stderr)

    # 3. Merge with existing ledger (idempotent), keep best record per id
    existing = load_existing_ledger()
    final_map = merge_records(
        {k: v for k, v in existing.items()},   # existing already-shaped lines
        {k: to_ledger_line(v) for k, v in qualified.items()},
    )

    lines = sorted(final_map.values(), key=lambda r: (r.get("date", ""), r.get("domain", "")))

    # stats (computed before write so --dry-run reports the same numbers)
    from collections import Counter
    by_domain = Counter(l["domain"] for l in lines)
    dates = [l["date"] for l in lines if l.get("date")]
    print(f"[ledger] total after merge: {len(lines)} lines", file=sys.stderr)
    print(f"[ledger] by domain: {dict(by_domain)}", file=sys.stderr)
    if dates:
        print(f"[ledger] span: {min(dates)} → {max(dates)}", file=sys.stderr)

    if args.dry_run:
        print("[ledger] --dry-run: nothing written", file=sys.stderr)
        for l in lines[-8:]:
            print(f"  {l['date']}  {l['domain']:<16} {l['signal'][:60]}", file=sys.stderr)
        return

    write_ledger(final_map)
    print(f"[ledger] wrote {len(lines)} lines → {LEDGER_PATH.relative_to(REPO_ROOT)}", file=sys.stderr)


if __name__ == "__main__":
    main()
