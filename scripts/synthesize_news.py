#!/usr/bin/env python3
"""
Weekly tech news synthesis generator — editorial quality version.
Reads "selected" articles (scored by score_articles.py) from the past 7 days.
Generates a bilingual long-form journalistic article (2000-3000 words, 10-15 min read).

Special handling:
  - Security articles → mandatory "Actions concrètes" section with concrete steps
  - Architecture article → Wikipedia architect bio + image
  - Previous syntheses → inject last 3 titles/angles to avoid repetition

Usage:
  python scripts/synthesize_news.py                        # current week
  python scripts/synthesize_news.py --week 2026-W22       # specific past week
  python scripts/synthesize_news.py --force                # ignore existing synthesis
"""
import argparse
import json
import os
import re
import sys
from datetime import datetime, timezone, timedelta, date
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


def parse_week(week_str: str) -> tuple[datetime, datetime]:
    """Parse '2026-W22' → (monday 00:00 UTC, sunday 23:59 UTC)."""
    year, w = week_str.split("-W")
    jan4 = date(int(year), 1, 4)
    week_start = jan4 + timedelta(weeks=int(w) - 1, days=-jan4.weekday())
    week_end   = week_start + timedelta(days=6)
    start_dt   = datetime(week_start.year, week_start.month, week_start.day, 0, 0, 0, tzinfo=timezone.utc)
    end_dt     = datetime(week_end.year, week_end.month, week_end.day, 23, 59, 59, tzinfo=timezone.utc)
    return start_dt, end_dt


def fetch_og_image(url: str) -> str:
    """Extract og:image meta tag from an article URL."""
    try:
        resp = requests.get(
            url, timeout=6,
            headers={"User-Agent": "Mozilla/5.0 (compatible; bourbask-veille/1.0)"},
            allow_redirects=True,
        )
        if resp.status_code != 200:
            return ""
        match = re.search(
            r'<meta[^>]+(?:property=["\']og:image["\']|name=["\']og:image["\'])[^>]+content=["\']([^"\']+)["\']',
            resp.text, re.IGNORECASE,
        )
        if not match:
            match = re.search(
                r'<meta[^>]+content=["\']([^"\']+)["\'][^>]+property=["\']og:image["\']',
                resp.text, re.IGNORECASE,
            )
        if match:
            img = match.group(1).strip()
            if img.startswith("http"):
                return img
    except Exception:
        pass
    return ""


def fetch_wikipedia_info(query: str) -> dict:
    """Fetch summary and thumbnail from Wikipedia REST API."""
    if not query.strip():
        return {}
    try:
        url = f"https://en.wikipedia.org/api/rest_v1/page/summary/{quote(query)}"
        resp = requests.get(
            url, timeout=8,
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
        # Fallback: opensearch
        search = requests.get(
            "https://en.wikipedia.org/w/api.php",
            params={"action": "opensearch", "search": query, "limit": 1, "format": "json"},
            timeout=8,
            headers={"User-Agent": "bourbask.github.io/veille-bot"},
        )
        if search.status_code == 200:
            results = search.json()
            if results[1]:
                return fetch_wikipedia_info(results[1][0])
    except Exception as e:
        print(f"[wikipedia] '{query}' failed: {e}", file=sys.stderr)
    return {}


def find_best_image(articles: list[dict]) -> dict:
    """
    Find the best illustration for the synthesis.
    Priority: top-scored selected article's og:image.
    Returns {"url": "...", "caption": "...", "source_title": "..."}
    """
    sorted_arts = sorted(articles, key=lambda a: a.get("score", 0) or 0, reverse=True)
    for art in sorted_arts[:5]:
        img = fetch_og_image(art.get("url", ""))
        if img:
            print(f"[image] Found OG image from: {art['source']} — {art['title'][:60]}")
            return {
                "url":          img,
                "caption":      art["title"],
                "source":       art.get("source", ""),
                "source_url":   art.get("url", ""),
            }
    return {}


def build_context(items: list[dict]) -> str:
    by_domain: dict[str, list[str]] = {}
    for item in items:
        domain = item.get("domain", item.get("categories", ["general"])[0])
        score  = item.get("score")
        score_str = f" [score:{score:.1f}]" if score else ""
        entry  = f'[{item.get("source","?")}][{item.get("lang","en")}]{score_str} {item["title"]}'
        by_domain.setdefault(domain, []).append(entry)

    lines = []
    for domain, titles in sorted(by_domain.items()):
        lines.append(f"\n## {domain.upper().replace('_', ' ')}")
        lines.extend(titles)
    return "\n".join(lines)


def generate_synthesis(
    client: anthropic.Anthropic,
    articles: list[dict],
    period_start: str,
    period_end: str,
    previous_syntheses: list[dict],
    week_id: str,
) -> dict | None:

    context = build_context(articles)

    # Previous syntheses context — avoid repeating angles
    prev_context = ""
    if previous_syntheses:
        prev_lines = []
        for s in previous_syntheses[:3]:
            prev_lines.append(
                f'  - {s["id"]}: "{s.get("title_fr","?")}" / "{s.get("title_en","?")}"'
            )
        prev_context = f"""
SYNTHÈSES PRÉCÉDENTES (à NE PAS répéter — angles déjà traités) :
{chr(10).join(prev_lines)}
Tu peux cependant glisser UNE courte phrase de rebond vers une synthèse précédente si un sujet s'y
connecte directement (ex: "Comme on l'évoquait en W22..."). Une seule, pas une liste."""

    security_present = any(
        a.get("domain") == "security" or "urgent" in a.get("categories", [])
        for a in articles
    )
    archi_present = any(a.get("domain") == "architecture" for a in articles)

    security_instruction = ""
    if security_present:
        security_instruction = """
SÉCURITÉ — OBLIGATOIRE :
L'article contient au moins un sujet de sécurité. Inclure une section dédiée avec :
- Explication claire de la menace (sans jargon inutile)
- Section "## Ce que tu dois faire maintenant" avec 3-5 actions PRÉCISES et IMMÉDIATES :
  commandes, versions à mettre à jour, configs à vérifier, liens directs.
  Pas de vague "mettez à jour vos systèmes". Sois chirurgical."""

    archi_instruction = ""
    if archi_present:
        archi_instruction = """
ARCHITECTURE — si un projet architectural est mentionné :
- Présenter brièvement l'architecte/studio (2-3 phrases max, pas de biographie)
- Expliquer l'innovation éco-responsable ou technique en termes journalistiques
- Dans le JSON, remplir "architecture_info" avec architect_name et search_query (pour Wikipedia)"""

    prompt = f"""Tu es Kevin Bourbasquet, et tu écris ta synthèse hebdomadaire de veille technologique.

TON ÉDITORIAL :
Kevin est développeur web senior, architecte de systèmes, passionné de souveraineté numérique,
sécurité web, écologie et architecture éco-responsable. Il écrit en français, directement, sans
jargon inutile ni ton corporate. Il aime prendre position. Il respecte l'ingénierie sérieuse,
démonte le hype, célèbre les vraies avancées open source. Quand quelque chose est important pour
la sécurité de ses lecteurs, il le dit clairement et donne des étapes concrètes.
Stack préféré : Rust, WebAssembly, open web platform, Linux.

SEMAINE : {period_start} au {period_end}
IDENTIFIANT : {week_id}

ARTICLES SÉLECTIONNÉS ({len(articles)} articles — gagnants de la compétition hebdomadaire) :
{context}
{prev_context}
{security_instruction}
{archi_instruction}

INSTRUCTIONS DE RÉDACTION :
Écris deux articles journalistiques complets, l'un en français (primaire), l'un en anglais.

Structure attendue (à adapter organiquement, pas une liste rigide) :
1. ACCROCHE — une scène, une anecdote, un paradoxe, une question provocatrice. PAS de résumé.
   Deux-trois paragraphes qui donnent envie de lire la suite.
2. DÉVELOPPEMENT — 3 à 5 sections thématiques avec ## titres. Chaque section développe un angle
   avec analyse, contexte historique si pertinent, implications pratiques. Pas juste des faits :
   POURQUOI ça compte, POUR QUI, QUEL impact à 6 mois.
3. CONNEXIONS — identifie le fil rouge de la semaine : quelle tendance de fond relie ces actualités ?
4. SÉCURITÉ (si applicable) — section dédiée avec actions concrètes (voir instruction ci-dessus)
5. ARCHITECTURE (si applicable) — présentation architecte + explication éco-technique
6. "ET MAINTENANT ?" — paragraphe de clôture : que doit surveiller le lecteur la semaine prochaine ?

CONTRAINTES :
- Longueur : 2000 à 3000 mots par article (10-15 minutes de lecture). Ne pas rogner.
- Markdown GFM propre. Commence directement par le premier heading.
- Mentionne des versions, CVE IDs, noms de projets réels — aucune généralité floue.
- Les titres FR et EN doivent être DIFFÉRENTS — deux angles éditoriaux distincts sur la même semaine.
- Style : The Verge ou MIT Technology Review, pas Medium. Pas de listes à puces pour les idées principales.

OUTPUT : JSON pur — aucun texte avant ou après, aucune fence markdown :
{{
  "title_fr": "Titre accrocheur (max 80 chars, style magazine)",
  "title_en": "Punchy English title (max 80 chars)",
  "content_fr": "# Titre\\n\\n[article FR complet 2000-3000 mots]",
  "content_en": "# Title\\n\\n[full English article 2000-3000 words]",
  "security_actions": {json.dumps(["Action précise 1", "Action précise 2"] if security_present else [])},
  "architecture_info": {{
    "present": {json.dumps(archi_present)},
    "architect_name": "",
    "project_name": "",
    "search_query": ""
  }}
}}"""

    try:
        response = client.messages.create(
            model="claude-sonnet-4-6",
            max_tokens=8000,
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
    parser = argparse.ArgumentParser()
    parser.add_argument("--week",  help="ISO week to synthesize, e.g. 2026-W22")
    parser.add_argument("--force", action="store_true", help="Overwrite existing synthesis for this week")
    args = parser.parse_args()

    api_key = os.environ.get("ANTHROPIC_API_KEY")
    if not api_key:
        print("[synthesis] ANTHROPIC_API_KEY not set", file=sys.stderr)
        sys.exit(1)

    out_path = Path(__file__).parent.parent / "public" / "news.json"
    if not out_path.exists():
        print("[synthesis] news.json not found", file=sys.stderr)
        sys.exit(1)

    data = json.loads(out_path.read_text(encoding="utf-8"))
    now  = datetime.now(timezone.utc)

    # Determine period
    if args.week:
        period_start_dt, period_end_dt = parse_week(args.week)
        synthesis_id = f"synthesis_{args.week.replace('-', '_')}"
    else:
        period_end_dt   = (now - timedelta(days=1)).replace(hour=23, minute=59, second=59, microsecond=0)
        period_start_dt = (now - timedelta(days=8)).replace(hour=0, minute=0, second=0, microsecond=0)
        synthesis_id    = isoweek_id(now)

    period_start = period_start_dt.strftime("%Y-%m-%d")
    period_end   = period_end_dt.strftime("%Y-%m-%d")
    print(f"Period: {period_start} → {period_end}  [{synthesis_id}]")

    # Check if synthesis already exists
    existing_synth = next((i for i in data["items"] if i.get("id") == synthesis_id), None)
    if existing_synth and not args.force:
        print(f"[synthesis] {synthesis_id} already exists. Use --force to overwrite.")
        sys.exit(0)

    # Load previous syntheses for context (exclude current week)
    previous_syntheses = [
        i for i in data["items"]
        if i.get("type") == "synthesis" and i.get("id") != synthesis_id
    ][:3]

    # Collect selected articles for this period
    candidate_articles = [
        item for item in data.get("items", [])
        if item.get("type", "article") == "article"
        and item.get("status") in ("selected", None)
        and period_start_dt.isoformat() <= item.get("published_at", "") <= period_end_dt.isoformat()
    ]

    if len(candidate_articles) < 3:
        print(f"[synthesis] Only {len(candidate_articles)} articles in window — need ≥ 3", file=sys.stderr)
        sys.exit(0)

    print(f"Synthesizing from {len(candidate_articles)} selected articles:")
    for a in candidate_articles:
        score_str = f" [{a.get('score', '?'):.1f}]" if isinstance(a.get('score'), float) else ""
        print(f"  [{a.get('domain','?')}]{score_str} {a['title'][:70]}")

    print(f"\nPrevious syntheses context: {[s['id'] for s in previous_syntheses]}")

    # Find best illustration
    print("\n[image] Searching for illustration…")
    illustration = find_best_image(candidate_articles)
    if not illustration:
        print("[image] No OG image found.")

    client = anthropic.Anthropic(api_key=api_key)
    print("\nGenerating synthesis (claude-sonnet-4-6, up to 8000 tokens)…")
    synthesis = generate_synthesis(
        client, candidate_articles, period_start, period_end,
        previous_syntheses, synthesis_id,
    )
    if not synthesis:
        print("[synthesis] Generation failed", file=sys.stderr)
        sys.exit(1)

    # Word count check
    fr_words = len(synthesis.get("content_fr", "").split())
    en_words = len(synthesis.get("content_en", "").split())
    print(f"[synthesis] Word count — FR: {fr_words}, EN: {en_words}")
    if fr_words < 1200:
        print(f"[synthesis] Warning: FR article seems short ({fr_words} words)")

    # Architecture visual
    archi_info   = synthesis.get("architecture_info", {})
    archi_visual = {}
    if archi_info.get("present") and archi_info.get("search_query"):
        print(f"[wikipedia] Fetching: {archi_info['search_query']}")
        archi_visual = fetch_wikipedia_info(archi_info["search_query"])
        if archi_visual.get("image_url"):
            print(f"[wikipedia] Image: {archi_visual['image_url'][:60]}…")

    # Remove existing synthesis if overwriting
    items = [i for i in data["items"] if i.get("id") != synthesis_id]

    # Tag source articles
    synthesized_ids = {a["id"] for a in candidate_articles}
    for item in items:
        if item.get("id") in synthesized_ids:
            item["synthesis_id"] = synthesis_id

    synthesis_card = {
        "id":                synthesis_id,
        "type":              "synthesis",
        "title_fr":          synthesis.get("title_fr", "Synthèse hebdomadaire"),
        "title_en":          synthesis.get("title_en", "Weekly Synthesis"),
        "period_start":      period_start,
        "period_end":        period_end,
        "published_at":      now.isoformat(),
        "content_fr":        synthesis.get("content_fr", ""),
        "content_en":        synthesis.get("content_en", ""),
        "security_actions":  synthesis.get("security_actions", []),
        "architecture_visual": archi_visual if archi_visual else None,
        "illustration":      illustration if illustration else None,
        "word_count":        {"fr": fr_words, "en": en_words},
        "source_count":      len(candidate_articles),
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

    # Insert at front
    data["generated_at"] = now.isoformat()
    data["items"]        = [synthesis_card] + items
    data["count"]        = len(data["items"])

    out_path.write_text(json.dumps(data, ensure_ascii=False, indent=2), encoding="utf-8")
    print(f"\nDone: {synthesis_id} — FR:{fr_words}w / EN:{en_words}w — {len(candidate_articles)} sources")
    if synthesis.get("security_actions"):
        print(f"Security actions: {len(synthesis['security_actions'])}")
    if illustration:
        print(f"Illustration: {illustration['url'][:70]}")


if __name__ == "__main__":
    main()
