#!/usr/bin/env python3
"""
One-shot migration script.
- Keeps only articles from weeks W21, W22, W23 (weeks with existing syntheses)
- Removes all existing synthesis cards
- Assigns domain from source name / categories (retroactive)
- Resets all kept articles to status="raw" so score_articles.py can process them
- Prints per-week summary

Run once, then:
  python scripts/score_articles.py --week 2026-W21
  python scripts/score_articles.py --week 2026-W22
  python scripts/score_articles.py --week 2026-W23
  python scripts/synthesize_news.py --week 2026-W21
  python scripts/synthesize_news.py --week 2026-W22
  python scripts/synthesize_news.py --week 2026-W23
"""
import json
import sys
from collections import defaultdict
from datetime import datetime, timezone
from pathlib import Path

# ── Domain inference ───────────────────────────────────────────────────────────
SOURCE_TO_DOMAIN: dict[str, str] = {
    # dev_stack
    "Rust Blog": "dev_stack", "This Week in Rust": "dev_stack",
    "Go Blog": "dev_stack", "Mozilla Hacks": "dev_stack",
    "WebKit Blog": "dev_stack", "GitHub Engineering": "dev_stack",
    "Linux Foundation": "dev_stack", "LWN.net": "dev_stack",
    "Lobste.rs": "dev_stack", "LinuxFr.org": "dev_stack",
    "Node.js Blog": "dev_stack", "Python Blog": "dev_stack",
    "PHP.net": "dev_stack", "Symfony Blog": "dev_stack",
    "React Blog": "dev_stack", "Chromium Blog": "dev_stack",
    "Framablog": "dev_stack",
    # ai_emerging
    "ArXiv CS.AI": "ai_emerging", "ArXiv CS.SE": "ai_emerging",
    "ArXiv CS.PL": "ai_emerging", "Papers With Code": "ai_emerging",
    "IEEE Spectrum": "ai_emerging", "ACM Tech News": "ai_emerging",
    # security
    "CISA Advisories": "security", "NIST CSRC": "security",
    "ENISA": "security", "OpenSSF Blog": "security",
    "Krebs on Security": "security", "Schneier on Security": "security",
    "PortSwigger Blog": "security", "SANS ISC": "security",
    "ArXiv CS.CR": "security",
    # health_science
    "ArXiv CS.HC": "health_science", "ArXiv q-bio": "health_science",
    "eLife Sciences": "health_science", "PLOS ONE": "health_science",
    # business_market
    "HackerNews": "business_market", "The Register": "business_market",
    "The Register Dev": "business_market", "InfoQ": "business_market",
    "CNCF Blog": "business_market", "W3C Blog": "business_market",
    "IETF Blog": "business_market",
    # architecture
    "Dezeen Sustainable": "architecture", "Low-tech Magazine": "architecture",
    "ArchDaily": "architecture", "TreeHugger": "architecture",
    "Resilient Design": "architecture",
}

CATEGORY_TO_DOMAIN: dict[str, str] = {
    "urgent": "security",
}


def infer_domain(item: dict) -> str:
    source = item.get("source", "")
    if source in SOURCE_TO_DOMAIN:
        return SOURCE_TO_DOMAIN[source]
    # Try partial match
    for k, v in SOURCE_TO_DOMAIN.items():
        if k.lower() in source.lower() or source.lower() in k.lower():
            return v
    # Fallback from categories
    for cat in item.get("categories", []):
        if cat in CATEGORY_TO_DOMAIN:
            return CATEGORY_TO_DOMAIN[cat]
    return "dev_stack"  # safest default


def isoweek(iso_str: str) -> str:
    try:
        dt = datetime.fromisoformat(iso_str[:19].replace("Z", "+00:00"))
        y, w, _ = dt.isocalendar()
        return f"{y}-W{w:02d}"
    except Exception:
        return "unknown"


def main() -> None:
    out_path = Path(__file__).parent.parent / "public" / "news.json"
    if not out_path.exists():
        print("news.json not found", file=sys.stderr)
        sys.exit(1)

    data  = json.loads(out_path.read_text(encoding="utf-8"))
    items = data.get("items", [])

    # Identify weeks that have synthesis cards
    synth_weeks: set[str] = set()
    for item in items:
        if item.get("type") == "synthesis":
            start = item.get("period_start", "")
            end   = item.get("period_end", "")
            if start:
                synth_weeks.add(isoweek(start))

    print(f"Synthesis weeks detected: {sorted(synth_weeks)}")
    if not synth_weeks:
        print("No synthesis cards found — nothing to migrate.")
        sys.exit(0)

    kept:    list[dict] = []
    dropped: int        = 0
    by_week: dict[str, list] = defaultdict(list)

    for item in items:
        if item.get("type") == "synthesis":
            dropped += 1
            continue  # drop all existing synthesis cards

        pub  = item.get("published_at", "")
        week = isoweek(pub)

        if week not in synth_weeks:
            dropped += 1
            continue

        # Assign domain retroactively
        item["domain"]        = infer_domain(item)
        item["status"]        = "raw"
        item["fetched_at"]    = item.get("fetched_at") or pub
        # Clear synthesis tag
        item.pop("synthesis_id", None)
        item.pop("score", None)
        item.pop("score_reason", None)

        kept.append(item)
        by_week[week].append(item)

    print(f"\nKept:    {len(kept)} articles")
    print(f"Dropped: {dropped} items (syntheses + out-of-scope articles)")
    print()
    for week in sorted(by_week.keys()):
        arts = by_week[week]
        domains: dict[str, int] = defaultdict(int)
        for a in arts:
            domains[a["domain"]] += 1
        print(f"  {week}: {len(arts)} articles  — {dict(sorted(domains.items()))}")

    print("\nNext steps:")
    for week in sorted(synth_weeks):
        print(f"  python scripts/score_articles.py --week {week}")
    print()
    for week in sorted(synth_weeks):
        print(f"  python scripts/synthesize_news.py --week {week}")

    data["items"]        = kept
    data["count"]        = len(kept)
    data["synthesis"]    = None
    data["generated_at"] = datetime.now(timezone.utc).isoformat()

    out_path.write_text(json.dumps(data, ensure_ascii=False, indent=2), encoding="utf-8")
    print(f"\nMigration done → {out_path}")


if __name__ == "__main__":
    main()
