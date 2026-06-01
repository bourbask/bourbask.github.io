#!/usr/bin/env python3
"""
Weekly tech news synthesis generator.
Reads "selected" articles from the past 7 days.
Generates a bilingual journalistic article with Kevin's editorial voice.

Special handling:
  - Security articles → mandatory "Actions concrètes" section
  - Architecture article → Wikipedia architect bio + image

Intended to run every Monday at 07:00 UTC via GitHub Actions.
"""
import json
import os
import re
import sys
from datetime import datetime, timezone, timedelta
from pathlib import Path
from urllib.parse import quote

try:
    import anthropic
    import requests
except ImportError:
    print("Missing deps: pip install anthropic requests", file=sys.stderr)
    sys.exit(1)


def isoweek_id(dt: datetime) -> str:
    year, week, _ = dt.isocalendar()
    return f"synthesis_{year}_W{week:02d}"


def fetch_wikipedia_info(query: str) -> dict:
    """Fetch summary and thumbnail from Wikipedia REST API."""
    if not query.strip():
        return {}
    try:
        url = f"https://en.wikipedia.org/api/rest_v1/page/summary/{quote(query)}"
        resp = requests.get(
            url,
            timeout=8,
            headers={"User-Agent": "bourbask.github.io/veille-bot (k.bourbasquet@legal2digital.fr)"},
        )
        if resp.status_code == 200:
            d = resp.json()
            return {
                "name":          query,
                "bio":           d.get("extract", "")[:600],
                "image_url":     d.get("thumbnail", {}).get("source", ""),
                "wikipedia_url": d.get("content_urls", {}).get("desktop", {}).get("page", ""),
            }
        # Try search if direct lookup failed
        search_url = "https://en.wikipedia.org/w/api.php"
        search_resp = requests.get(
            search_url,
            params={"action": "opensearch", "search": query, "limit": 1, "format": "json"},
            timeout=8,
            headers={"User-Agent": "bourbask.github.io/veille-bot"},
        )
        if search_resp.status_code == 200:
            results = search_resp.json()
            if results[1]:
                return fetch_wikipedia_info(results[1][0])
    except Exception as e:
        print(f"[wikipedia] query '{query}' failed: {e}", file=sys.stderr)
    return {}


def build_context(items: list[dict]) -> str:
    """Build article context grouped by domain for the synthesis prompt."""
    by_domain: dict[str, list[str]] = {}
    for item in items:
        domain = item.get("domain", item.get("categories", ["general"])[0])
        entry  = f'[{item.get("source", "?")}][{item.get("lang", "en")}] {item["title"]}'
        if item.get("score"):
            entry += f' (score: {item["score"]:.1f})'
        by_domain.setdefault(domain, []).append(entry)

    lines = []
    for domain, titles in sorted(by_domain.items()):
        lines.append(f"\n## {domain.upper().replace('_', ' ')}")
        lines.extend(titles)
    return "\n".join(lines)


def has_security(items: list[dict]) -> bool:
    return any(
        it.get("domain") == "security"
        or "urgent" in it.get("categories", [])
        for it in items
    )


def has_architecture(items: list[dict]) -> bool:
    return any(it.get("domain") == "architecture" for it in items)


def generate_synthesis(
    client: anthropic.Anthropic,
    items: list[dict],
    period_start: str,
    period_end: str,
) -> dict | None:
    context      = build_context(items)
    security_req = has_security(items)
    archi_req    = has_architecture(items)

    security_instruction = ""
    if security_req:
        security_instruction = """
SECURITY ARTICLES PRESENT — MANDATORY section in both articles:
After discussing any security topic, add a "## Actions concrètes" section listing
3-5 specific, immediately actionable steps a developer can take RIGHT NOW
(e.g. "mettre à jour OpenSSL vers 3.x", "vérifier votre nginx.conf pour l'option X",
"activer MFA sur votre compte GitHub"). Be precise: include version numbers, command lines,
or config snippets when relevant. No vague advice."""

    archi_instruction = ""
    if archi_req:
        archi_instruction = """
ARCHITECTURE ARTICLE PRESENT — for any architecture/building project mentioned:
In the JSON, populate "architecture_info" with:
- architect_name: the architect(s) or studio name (exact Wikipedia-searchable name)
- project_name: the building/project name
- search_query: the best Wikipedia search query to find the architect (in English)
If no specific architect is named in the source, set architect_name to "" and search_query to ""."""

    prompt = f"""You are writing Kevin Bourbasquet's personal tech watch weekly synthesis.

EDITORIAL VOICE — Kevin is:
- A French senior web developer and system architect
- Values: digital sovereignty (own your stack), web security as first-class concern, ecological responsibility
- Stack: Rust, WebAssembly, open web platform, Linux
- Tone: direct, technically precise, slightly opinionated, zero corporate BS
  Think "someone who reads Le Monde and writes Rust in the same afternoon"
- He respects genuine engineering, calls out hype, celebrates real open-source progress
- When something is important for security, he says it plainly and gives concrete steps
- On architecture: he connects eco-responsible building to the same values as digital sovereignty

WEEK: {period_start} to {period_end}
SELECTED ARTICLES ({len(items)} articles that won their domain competition):
{context}
{security_instruction}
{archi_instruction}

Write two full editorial articles (FR primary + EN):
- OPEN with a narrative hook or a punchy statement — NOT a bullet list summary
- Use ## subheadings organically when topics shift
- Take positions. Explain WHY things matter, not just WHAT happened
- Identify the week's signal — what is the underlying trend?
- Close with "Et maintenant ?" / "So what?" — concrete takeaway for the reader
- 600-900 words per article
- Clean Markdown (GFM). No YAML frontmatter. Start directly with the first heading.

OUTPUT: pure JSON only — no markdown fences, no text before or after:
{{
  "title_fr": "Titre accrocheur (max 80 chars, style magazine)",
  "title_en": "Punchy English title (max 80 chars)",
  "content_fr": "# Titre\\n\\n[article complet en français]",
  "content_en": "# Title\\n\\n[full English article]",
  "security_actions": {json.dumps(["Action 1", "Action 2"] if security_req else [])},
  "architecture_info": {{
    "present": {json.dumps(archi_req)},
    "architect_name": "",
    "project_name": "",
    "search_query": ""
  }}
}}

RULES:
- title_fr ≠ title_en — two distinct editorial angles on the same week
- content_fr must be in French, content_en in English
- Mention specific version numbers, CVE IDs, project names — no vague generalities
- security_actions: {json.dumps("list of 3-5 precise actionable steps (fr)" if security_req else "empty array (no security articles this week)")}
- Respond ONLY with valid JSON"""

    try:
        response = client.messages.create(
            model="claude-sonnet-4-6",
            max_tokens=5000,
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

    api_key = os.environ.get("ANTHROPIC_API_KEY")
    if not api_key:
        print("[synthesis] ANTHROPIC_API_KEY not set", file=sys.stderr)
        sys.exit(1)

    data = json.loads(out_path.read_text(encoding="utf-8"))
    now  = datetime.now(timezone.utc)

    period_end_dt   = (now - timedelta(days=1)).replace(hour=23, minute=59, second=59, microsecond=0)
    period_start_dt = (now - timedelta(days=8)).replace(hour=0, minute=0, second=0, microsecond=0)
    period_start    = period_start_dt.strftime("%Y-%m-%d")
    period_end      = period_end_dt.strftime("%Y-%m-%d")

    print(f"Period: {period_start} → {period_end}")

    # Only use "selected" articles in the window
    candidate_articles = [
        item for item in data.get("items", [])
        if item.get("type", "article") == "article"
        and item.get("status") in ("selected", None)   # None = legacy pre-scoring
        and period_start_dt.isoformat() <= item.get("published_at", "") <= period_end_dt.isoformat()
    ]

    if len(candidate_articles) < 3:
        print(
            f"[synthesis] Only {len(candidate_articles)} selected articles in window — aborting (need ≥ 3)",
            file=sys.stderr,
        )
        sys.exit(0)

    print(f"Generating synthesis over {len(candidate_articles)} selected articles…")
    for it in candidate_articles:
        print(f"  [{it.get('domain', '?')}] {it['title'][:70]}")

    client = anthropic.Anthropic(api_key=api_key)
    synthesis = generate_synthesis(client, candidate_articles, period_start, period_end)

    if not synthesis:
        print("[synthesis] Generation failed, aborting", file=sys.stderr)
        sys.exit(1)

    # Enrich architecture info via Wikipedia
    archi_info    = synthesis.get("architecture_info", {})
    archi_visual  = {}
    if archi_info.get("present") and archi_info.get("search_query"):
        print(f"[wikipedia] Fetching architect info: {archi_info['search_query']}")
        archi_visual = fetch_wikipedia_info(archi_info["search_query"])
        if archi_visual.get("image_url"):
            print(f"[wikipedia] Image found: {archi_visual['image_url'][:60]}…")
        else:
            print("[wikipedia] No image found.")

    synthesis_id = isoweek_id(now)

    # Tag source articles
    synthesized_ids = {a["id"] for a in candidate_articles}
    for item in data.get("items", []):
        if item.get("id") in synthesized_ids:
            item["synthesis_id"] = synthesis_id

    synthesis_card = {
        "id":              synthesis_id,
        "type":            "synthesis",
        "title_fr":        synthesis.get("title_fr", "Synthèse hebdomadaire"),
        "title_en":        synthesis.get("title_en", "Weekly Synthesis"),
        "period_start":    period_start,
        "period_end":      period_end,
        "published_at":    now.isoformat(),
        "content_fr":      synthesis.get("content_fr", ""),
        "content_en":      synthesis.get("content_en", ""),
        "security_actions": synthesis.get("security_actions", []),
        "architecture_visual": archi_visual if archi_visual else None,
        "source_count":    len(candidate_articles),
        "sources": [
            {
                "id":           a["id"],
                "title":        a["title"],
                "url":          a.get("url", ""),
                "source":       a.get("source", ""),
                "domain":       a.get("domain", ""),
                "lang":         a.get("lang", "en"),
                "published_at": a.get("published_at", ""),
                "score":        a.get("score"),
            }
            for a in candidate_articles
        ],
    }

    # Insert synthesis card at front
    data["generated_at"] = now.isoformat()
    data["items"] = [synthesis_card] + [
        item for item in data.get("items", [])
        if item.get("id") != synthesis_id
    ]
    data["count"] = len(data["items"])

    out_path.write_text(json.dumps(data, ensure_ascii=False, indent=2), encoding="utf-8")
    print(
        f"Done: synthesis card {synthesis_id} created, "
        f"{len(candidate_articles)} articles tagged, {len(data['items'])} total items"
    )
    if synthesis.get("security_actions"):
        print(f"Security actions: {len(synthesis['security_actions'])} items included")
    if archi_visual.get("image_url"):
        print(f"Architecture visual: {archi_info.get('architect_name')} — {archi_visual['image_url'][:60]}")


if __name__ == "__main__":
    main()
