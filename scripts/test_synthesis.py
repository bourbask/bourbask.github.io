#!/usr/bin/env python3
"""
Local test script for synthesis prompt iteration.
Loads news.json, takes articles from the past N days, calls Claude Haiku,
prints output JSON (title_fr, title_en, content_fr, content_en).
Does NOT write to news.json.

Usage: ANTHROPIC_API_KEY=... python scripts/test_synthesis.py [--days 7]
"""
import json
import os
import re
import sys
from datetime import datetime, timezone, timedelta
from pathlib import Path


def _load_dotenv() -> None:
    env = Path(__file__).parent.parent / ".env"
    if not env.exists():
        return
    for line in env.read_text(encoding="utf-8").splitlines():
        line = line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        k, _, v = line.partition("=")
        k = k.strip()
        v = v.strip().strip('"').strip("'")
        if k and k not in os.environ:
            os.environ[k] = v

_load_dotenv()

try:
    import anthropic
except ImportError:
    print("Missing dep: pip install anthropic", file=sys.stderr)
    sys.exit(1)

DAYS = int(sys.argv[sys.argv.index("--days") + 1]) if "--days" in sys.argv else 7


def main() -> None:
    news_path = Path(__file__).parent.parent / "public" / "news.json"
    if not news_path.exists():
        print("news.json not found", file=sys.stderr)
        sys.exit(1)

    api_key = os.environ.get("ANTHROPIC_API_KEY")
    if not api_key:
        print("ANTHROPIC_API_KEY not set", file=sys.stderr)
        sys.exit(1)

    data = json.loads(news_path.read_text(encoding="utf-8"))
    now = datetime.now(timezone.utc)

    period_end_dt = (now - timedelta(days=1)).replace(hour=23, minute=59, second=59, microsecond=0)
    period_start_dt = (now - timedelta(days=DAYS + 1)).replace(hour=0, minute=0, second=0, microsecond=0)
    period_start = period_start_dt.strftime("%Y-%m-%d")
    period_end = period_end_dt.strftime("%Y-%m-%d")

    articles = [
        item for item in data.get("items", [])
        if item.get("type", "article") == "article"
        and period_start_dt.isoformat() <= item.get("published_at", "") <= period_end_dt.isoformat()
    ]

    print(f"Period: {period_start} → {period_end}")
    print(f"Articles in window: {len(articles)}")

    if len(articles) < 5:
        print(f"Only {len(articles)} articles — proceeding anyway (test mode)")

    by_cat: dict[str, list[str]] = {}
    for item in articles:
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

SOURCE MATERIAL ({len(articles)} articles from institutional, academic, and curated sources):
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

    print("\nCalling Claude Haiku…")
    client = anthropic.Anthropic(api_key=api_key)
    response = client.messages.create(
        model="claude-haiku-4-5-20251001",
        max_tokens=4000,
        messages=[{"role": "user", "content": prompt}],
    )

    text = response.content[0].text.strip()
    text = re.sub(r"^```(?:json)?\s*\n?", "", text)
    text = re.sub(r"\n?```\s*$", "", text)

    try:
        result = json.loads(text)
    except json.JSONDecodeError as e:
        print(f"\nJSON parse error: {e}", file=sys.stderr)
        print("\nRaw output:")
        print(text)
        sys.exit(1)

    print(f"\n--- title_fr ---\n{result.get('title_fr', 'MISSING')}")
    print(f"\n--- title_en ---\n{result.get('title_en', 'MISSING')}")
    print(f"\n--- content_fr (first 500 chars) ---\n{result.get('content_fr', 'MISSING')[:500]}…")
    print(f"\n--- content_en (first 500 chars) ---\n{result.get('content_en', 'MISSING')[:500]}…")
    print(f"\nInput tokens: {response.usage.input_tokens}, Output tokens: {response.usage.output_tokens}")


if __name__ == "__main__":
    main()
