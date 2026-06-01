#!/usr/bin/env python3
"""
Article competition scoring.
Groups "raw" articles by domain, scores them with Claude Haiku,
selects top 3 per domain then top 5 globally (max 1 architecture).

Usage:
  python scripts/score_articles.py                  # score today's raw articles
  python scripts/score_articles.py --date 2026-06-01
  python scripts/score_articles.py --week 2026-W22  # score all raw from that ISO week
  python scripts/score_articles.py --dry-run        # print results, don't write
"""
import argparse
import json
import os
import re
import sys
from datetime import datetime, timezone, timedelta, date
from pathlib import Path

try:
    import anthropic
except ImportError:
    print("Missing dep: pip install anthropic", file=sys.stderr)
    sys.exit(1)

GLOBAL_TOP           = 5
TOP_PER_DOMAIN       = 3
MAX_CANDIDATES       = 15   # max articles sent to Claude per domain scoring
MAX_ARCH_IN_TOP      = 1    # max architecture articles in global top 5

# Scoring prompt per domain
DOMAIN_CRITERIA: dict[str, str] = {
    "dev_stack": (
        "You are evaluating articles for a senior developer who uses Rust, WebAssembly, "
        "and open source web tooling daily. "
        "Score each article on: Stack relevance (Rust/WASM/web/open source) 40%, "
        "Technical importance and developer impact 40%, "
        "Genuine signal vs hype/marketing 20%. "
        "Penalise marketing posts. Reward technical depth and concrete releases."
    ),
    "ai_emerging": (
        "You are a skeptical technical practitioner evaluating AI/emerging tech articles. "
        "Score on: Practical real-world applicability (not just benchmarks) 40%, "
        "Scientific rigor and genuine breakthrough 40%, "
        "Novelty vs incremental hype 20%. "
        "Heavily penalise LLM hype without technical substance. "
        "Reward peer-reviewed results, open-source releases, reproducible findings."
    ),
    "security": (
        "You are evaluating security articles for web developers who need to take action. "
        "Score on: Severity and urgency for typical web/app developers 50%, "
        "Actionability — is there a concrete patch/mitigation/configuration? 30%, "
        "Breadth of impact — how many developers are affected? 20%. "
        "Prefer articles with CVE IDs, clear mitigations, and affected version ranges."
    ),
    "health_science": (
        "You are evaluating science/health/biomechanics research for general interest. "
        "Score on: Genuine advancement for human health or wellbeing 40%, "
        "Scientific rigor and innovation level 40%, "
        "Accessibility and relevance of the findings for non-specialists 20%. "
        "Prefer peer-reviewed publications over press releases."
    ),
    "business_market": (
        "You are evaluating tech business/market articles for long-term signal. "
        "Score on: Long-term strategic signal (not short-term noise or hype) 40%, "
        "Developer ecosystem and open source impact 30%, "
        "Surprising or contrarian insight that challenges conventional wisdom 30%. "
        "Penalise pure funding announcements without technical substance. "
        "Reward analysis pieces, market shifts, and ecosystem trends."
    ),
    "architecture": (
        "You are evaluating architecture articles for someone planning to self-build "
        "an eco-responsible home in France. "
        "Score on: Eco-responsibility and climate adaptation (passive cooling/heating, "
        "rising temperatures, extreme weather) 40%, "
        "DIY/self-construction applicability with local/natural materials "
        "(rammed earth, straw, wood, hemp, stone) 40%, "
        "Innovation in water management, energy autonomy, or food integration 20%. "
        "Penalise luxury architecture without sustainability substance. "
        "Reward Passivhaus, earthship, low-tech, permaculture-integrated designs."
    ),
}


def parse_week(week_str: str) -> tuple[date, date]:
    """Parse '2026-W22' → (monday, sunday) as date objects."""
    year, w = week_str.split("-W")
    jan4 = date(int(year), 1, 4)
    week_start = jan4 + timedelta(weeks=int(w) - 1, days=-jan4.weekday())
    week_end   = week_start + timedelta(days=6)
    return week_start, week_end


def score_domain(
    client: anthropic.Anthropic,
    domain: str,
    articles: list[dict],
) -> list[dict]:
    """
    Ask Claude Haiku to score and rank articles for a given domain.
    Returns articles list with '_score' and '_score_reason' fields added, sorted desc.
    Falls back to position order if API call fails.
    """
    criteria = DOMAIN_CRITERIA.get(domain, "Score by general importance and quality.")
    candidates = articles[:MAX_CANDIDATES]

    lines = []
    for i, art in enumerate(candidates):
        lines.append(f'{i+1}. [ID:{art["id"]}] [{art["source"]}] {art["title"]}')
    article_list = "\n".join(lines)

    prompt = f"""You are scoring {len(candidates)} articles for the domain "{domain}".

SCORING CRITERIA:
{criteria}

ARTICLES:
{article_list}

Return ONLY a valid JSON array — no markdown, no text before or after:
[
  {{"id": "article_id", "score": 8.5, "reason": "one sentence why"}},
  ...
]

Rules:
- Score each article from 0.0 to 10.0
- Include ALL {len(candidates)} articles
- "reason" must be ≤15 words, factual, no fluff
- Sort by score descending in your response"""

    try:
        response = client.messages.create(
            model="claude-haiku-4-5-20251001",
            max_tokens=1500,
            messages=[{"role": "user", "content": prompt}],
        )
        text = response.content[0].text.strip()
        text = re.sub(r"^```(?:json)?\s*\n?", "", text)
        text = re.sub(r"\n?```\s*$", "", text)
        scores: list[dict] = json.loads(text)

        score_map = {s["id"]: s for s in scores}
        for art in candidates:
            s = score_map.get(art["id"], {})
            art["_score"]        = float(s.get("score", 0.0))
            art["_score_reason"] = s.get("reason", "")

        candidates.sort(key=lambda x: x.get("_score", 0.0), reverse=True)
        return candidates

    except Exception as e:
        print(f"[score] {domain} scoring failed: {e}", file=sys.stderr)
        # Fallback: assign decreasing scores by position
        for i, art in enumerate(candidates):
            art["_score"]        = float(max(0, 5.0 - i * 0.3))
            art["_score_reason"] = "fallback order"
        return candidates


def select_global_top5(domain_winners: dict[str, list[dict]]) -> list[str]:
    """
    From top-3 per domain, select global top 5.
    Constraint: max MAX_ARCH_IN_TOP architecture articles.
    Returns list of selected article IDs.
    """
    candidates = []
    for domain, articles in domain_winners.items():
        for art in articles[:TOP_PER_DOMAIN]:
            candidates.append({
                "id":     art["id"],
                "domain": domain,
                "score":  art.get("_score", 0.0),
            })

    candidates.sort(key=lambda x: x["score"], reverse=True)

    selected: list[str] = []
    arch_count = 0

    for c in candidates:
        if len(selected) >= GLOBAL_TOP:
            break
        if c["domain"] == "architecture":
            if arch_count >= MAX_ARCH_IN_TOP:
                continue
            arch_count += 1
        selected.append(c["id"])

    return selected


def main() -> None:
    parser = argparse.ArgumentParser(description="Score and select daily top articles.")
    parser.add_argument("--date",    help="Target date YYYY-MM-DD (default: today)")
    parser.add_argument("--week",    help="Target ISO week YYYY-WNN (score all raw from that week)")
    parser.add_argument("--dry-run", action="store_true", help="Print results without writing")
    args = parser.parse_args()

    api_key = os.environ.get("ANTHROPIC_API_KEY")
    if not api_key:
        print("[score] ANTHROPIC_API_KEY not set", file=sys.stderr)
        sys.exit(1)

    out_path = Path(__file__).parent.parent / "public" / "news.json"
    if not out_path.exists():
        print("[score] news.json not found", file=sys.stderr)
        sys.exit(1)

    data = json.loads(out_path.read_text(encoding="utf-8"))
    items: list[dict] = data.get("items", [])

    # Determine the target date range
    if args.week:
        week_start, week_end = parse_week(args.week)
        target_start = datetime(week_start.year, week_start.month, week_start.day, tzinfo=timezone.utc).isoformat()
        target_end   = datetime(week_end.year, week_end.month, week_end.day, 23, 59, 59, tzinfo=timezone.utc).isoformat()
        label = args.week
    else:
        target_date  = args.date or datetime.now(timezone.utc).strftime("%Y-%m-%d")
        target_start = f"{target_date}T00:00:00+00:00"
        target_end   = f"{target_date}T23:59:59+00:00"
        label = target_date

    print(f"Scoring raw articles for {label} ({target_start[:10]} → {target_end[:10]})…")

    # Collect raw articles in the target window
    # Match on fetched_at if present, else published_at
    raw_articles = [
        it for it in items
        if it.get("status") == "raw"
        and it.get("type", "article") == "article"
        and target_start <= (it.get("fetched_at") or it.get("published_at", "")) <= target_end
    ]

    if not raw_articles:
        print(f"[score] No raw articles found for {label}. Nothing to score.")
        sys.exit(0)

    print(f"Found {len(raw_articles)} raw articles to score.")

    # Group by domain
    by_domain: dict[str, list[dict]] = {}
    for art in raw_articles:
        d = art.get("domain", "dev_stack")
        by_domain.setdefault(d, []).append(art)

    for domain, arts in sorted(by_domain.items()):
        print(f"  {domain}: {len(arts)} articles")

    client = anthropic.Anthropic(api_key=api_key)

    domain_winners: dict[str, list[dict]] = {}
    for domain, arts in by_domain.items():
        if len(arts) == 0:
            continue
        print(f"\nScoring {domain} ({len(arts)} articles)…")
        scored = score_domain(client, domain, arts)
        domain_winners[domain] = scored
        top3 = scored[:TOP_PER_DOMAIN]
        for rank, art in enumerate(top3, 1):
            print(f"  #{rank} [{art['_score']:.1f}] {art['title'][:80]}")
            print(f"        → {art['_score_reason']}")

    # Global top 5
    selected_ids = set(select_global_top5(domain_winners))
    print(f"\n{'='*60}")
    print(f"Global top {GLOBAL_TOP} selected:")

    all_scored = [art for arts in domain_winners.values() for art in arts]
    selected_arts = [a for a in all_scored if a["id"] in selected_ids]
    selected_arts.sort(key=lambda x: x.get("_score", 0.0), reverse=True)
    for rank, art in enumerate(selected_arts, 1):
        print(f"  #{rank} [{art.get('_score', 0):.1f}] [{art.get('domain')}] {art['title'][:75]}")

    if args.dry_run:
        print("\n[dry-run] Not writing to news.json.")
        return

    # Update items in news.json
    # Build lookup: id → scored article
    score_lookup: dict[str, dict] = {a["id"]: a for a in all_scored}

    updated_count = 0
    for item in items:
        if item.get("id") in score_lookup:
            scored = score_lookup[item["id"]]
            new_status = "selected" if item["id"] in selected_ids else "archived"
            item["status"]        = new_status
            item["score"]         = scored.get("_score")
            item["score_reason"]  = scored.get("_score_reason", "")
            updated_count += 1

    data["items"]        = items
    data["count"]        = len(items)
    data["generated_at"] = datetime.now(timezone.utc).isoformat()

    out_path.write_text(json.dumps(data, ensure_ascii=False, indent=2), encoding="utf-8")
    print(f"\nDone: {updated_count} articles updated, {len(selected_ids)} selected.")


if __name__ == "__main__":
    main()
