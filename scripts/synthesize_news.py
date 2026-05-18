#!/usr/bin/env python3
"""
Weekly tech news synthesis generator.
Reads articles from public/news.json (past 7 days),
generates two bilingual Markdown articles (FR + EN) in the style of The Verge/Medium,
tags source articles with the synthesis ID, and inserts a synthesis card.

Intended to run every Monday at 07:00 UTC via GitHub Actions.
"""
import json
import os
import re
import sys
from datetime import datetime, timezone, timedelta
from pathlib import Path

try:
    import anthropic
except ImportError:
    print("Missing dep: pip install anthropic", file=sys.stderr)
    sys.exit(1)


def isoweek_id(dt: datetime) -> str:
    year, week, _ = dt.isocalendar()
    return f"synthesis_{year}_W{week:02d}"


def generate_weekly_synthesis(
    items: list[dict], period_start: str, period_end: str
) -> dict | None:
    api_key = os.environ.get("ANTHROPIC_API_KEY")
    if not api_key:
        print("[synthesis] ANTHROPIC_API_KEY not set", file=sys.stderr)
        return None

    client = anthropic.Anthropic(api_key=api_key)

    by_cat: dict[str, list[str]] = {}
    for item in items:
        for cat in item.get("categories", ["general"]):
            by_cat.setdefault(cat, []).append(
                f"[{item.get('source', '?')}][{item.get('lang', 'en')}] {item['title']}"
            )

    context_lines: list[str] = []
    for cat, titles in by_cat.items():
        context_lines.append(f"\n## {cat.upper()}")
        context_lines.extend(titles[:15])
    context = "\n".join(context_lines)

    prompt = f"""You are a tech journalist writing for a publication like The Verge or MIT Technology Review.
Your task: write two full editorial articles (FR + EN) summarizing the most important tech developments from the week of {period_start} to {period_end}.

SOURCE MATERIAL ({len(items)} articles from institutional, academic, and curated sources):
{context}

Write editorial articles with this style:
- OPEN with a compelling narrative hook or provocative statement — NOT a summary bullet list
- Use ## subheadings organically when topics shift, not as a rigid framework
- Include ONE markdown table when comparing competing technologies, versions, or release timelines
- Mix your own analysis with facts: take positions, explain why things matter to developers
- Identify the week's TREND, not just enumerate events
- Close with a "So what?" paragraph: what should a developer actually DO or WATCH this week?
- Write 500-800 words per article
- Format: clean Markdown (GFM). No YAML frontmatter. Start directly with the first heading.

OUTPUT: A JSON object — no markdown fences:
{{
  "title_fr": "Titre accrocheur en français (max 80 chars, style magazine)",
  "title_en": "Punchy English title (max 80 chars, magazine style)",
  "content_fr": "# Titre article FR\\n\\n[article complet en français, 500-800 mots]",
  "content_en": "# English Article Title\\n\\n[full article in English, 500-800 words]"
}}

RULES:
- title_fr and title_en must differ — two editorial angles on the same week
- content_fr must be in French, content_en in English
- Be critical: call out hype, celebrate genuine releases
- Mention specific version numbers, CVE IDs, real project names
- Respond ONLY with valid JSON, no text before or after"""

    try:
        response = client.messages.create(
            model="claude-haiku-4-5-20251001",
            max_tokens=4000,
            messages=[{"role": "user", "content": prompt}],
        )
        text = response.content[0].text.strip()
        text = re.sub(r"^```(?:json)?\s*\n?", "", text)
        text = re.sub(r"\n?```\s*$", "", text)
        return json.loads(text)
    except Exception as e:
        print(f"[synthesis] generation failed: {e}", file=sys.stderr)
        return None


def main() -> None:
    out_path = Path(__file__).parent.parent / "public" / "news.json"
    if not out_path.exists():
        print("[synthesis] news.json not found", file=sys.stderr)
        sys.exit(1)

    data = json.loads(out_path.read_text(encoding="utf-8"))
    now = datetime.now(timezone.utc)

    period_end_dt = (now - timedelta(days=1)).replace(
        hour=23, minute=59, second=59, microsecond=0
    )
    period_start_dt = (now - timedelta(days=8)).replace(
        hour=0, minute=0, second=0, microsecond=0
    )
    period_start = period_start_dt.strftime("%Y-%m-%d")
    period_end = period_end_dt.strftime("%Y-%m-%d")

    print(f"Period: {period_start} → {period_end}")

    candidate_articles = [
        item for item in data.get("items", [])
        if item.get("type", "article") == "article"
        and period_start_dt.isoformat() <= item.get("published_at", "") <= period_end_dt.isoformat()
    ]

    if len(candidate_articles) < 10:
        print(
            f"[synthesis] Only {len(candidate_articles)} articles in window — aborting (need ≥ 10)",
            file=sys.stderr,
        )
        sys.exit(0)

    print(f"Generating synthesis over {len(candidate_articles)} articles…")
    synthesis = generate_weekly_synthesis(candidate_articles, period_start, period_end)
    if not synthesis:
        print("[synthesis] Generation failed, aborting", file=sys.stderr)
        sys.exit(1)

    synthesis_id = isoweek_id(now)

    # Tag source articles (keep them in feed, just mark them)
    synthesized_ids = {a["id"] for a in candidate_articles}
    for item in data.get("items", []):
        if item.get("id") in synthesized_ids:
            item["synthesis_id"] = synthesis_id

    synthesis_card = {
        "id": synthesis_id,
        "type": "synthesis",
        "title_fr": synthesis.get("title_fr", "Synthèse hebdomadaire"),
        "title_en": synthesis.get("title_en", "Weekly Synthesis"),
        "period_start": period_start,
        "period_end": period_end,
        "published_at": now.isoformat(),
        "content_fr": synthesis.get("content_fr", ""),
        "content_en": synthesis.get("content_en", ""),
        "source_count": len(candidate_articles),
        "sources": [
            {
                "id": a["id"],
                "title": a["title"],
                "url": a.get("url", ""),
                "source": a.get("source", ""),
                "lang": a.get("lang", "en"),
                "published_at": a.get("published_at", ""),
            }
            for a in candidate_articles
        ],
    }

    # Insert synthesis card at front, keep all existing items (tagged or not)
    data["generated_at"] = now.isoformat()
    data["items"] = [synthesis_card] + [
        item for item in data.get("items", [])
        if item.get("id") != synthesis_id  # don't duplicate if re-run
    ]
    data["count"] = len(data["items"])

    out_path.write_text(json.dumps(data, ensure_ascii=False, indent=2), encoding="utf-8")
    print(
        f"Done: synthesis card {synthesis_id} created, "
        f"{len(candidate_articles)} articles tagged, {len(data['items'])} total items"
    )


if __name__ == "__main__":
    main()
