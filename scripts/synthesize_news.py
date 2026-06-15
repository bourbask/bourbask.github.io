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


def find_images(articles: list[dict], max_images: int = 3) -> list[dict]:
    """
    Fetch OG images from top-scored articles.
    Returns list of {url, caption, source, source_url, article_title}.
    """
    sorted_arts = sorted(articles, key=lambda a: a.get("score", 0) or 0, reverse=True)
    images = []
    seen_domains: set[str] = set()
    for art in sorted_arts:
        if len(images) >= max_images:
            break
        domain = art.get("domain", "")
        if domain in seen_domains:
            continue  # one image per domain max
        img = fetch_og_image(art.get("url", ""))
        if img:
            print(f"[image] Found OG image: [{domain}] {art['source']} — {art['title'][:55]}")
            images.append({
                "url":           img,
                "caption":       art["title"],
                "source":        art.get("source", ""),
                "source_url":    art.get("url", ""),
                "article_title": art["title"],
                "domain":        domain,
            })
            seen_domains.add(domain)
    return images


def proofread_french(client: anthropic.Anthropic, text: str) -> str:
    """
    Quick proofreading pass on French content using Claude Haiku.
    Fixes calques anglais, robotic phrasing, unnatural constructions.
    Preserves all markdown, links, images, and structure.
    """
    prompt = f"""Tu es un correcteur de style pour un magazine tech francophone.
Ton unique tâche : corriger les formulations maladroites, les calques de l'anglais,
et les tournures robotiques dans ce texte en français.

RÈGLES STRICTES :
— Ne change PAS le sens, les faits, les opinions, les noms propres, les chiffres
— Ne change PAS la structure, les headings, les hyperliens, les images, les tableaux, les blocs de code
— Corrige UNIQUEMENT les phrases qui sonnent faux en français natif :
  · Calques anglais ("c'est la même logique" → "c'est le même principe", etc.)
  · Constructions trop rigides ou trop nominales
  · Répétitions de mots à courte distance
  · Formules de transition mécaniques
— Si une phrase est déjà bonne, laisse-la intacte
— Réponds avec le texte corrigé uniquement, sans commentaire ni explication

TEXTE À CORRIGER :
{text}"""

    try:
        resp = client.messages.create(
            model="claude-haiku-4-5-20251001",
            max_tokens=6000,
            messages=[{"role": "user", "content": prompt}],
        )
        return resp.content[0].text.strip()
    except Exception as e:
        print(f"[proofread] Failed: {e}", file=sys.stderr)
        return text  # fallback: return original unchanged


def validate_images(client: anthropic.Anthropic, content: str, context_summary: str) -> str:
    """
    Review each ![alt](url) in the content.
    Remove images that don't clearly match the surrounding section topic.
    When in doubt: remove. Never keep a dubious image.
    Prefer a table or nothing over an irrelevant image.
    """
    import re
    images = re.findall(r'!\[([^\]]*)\]\(([^)]+)\)', content)
    if not images:
        return content

    prompt = f"""Tu évalues les illustrations d'un article journalistique.

CONTEXTE DE L'ARTICLE : {context_summary}

IMAGES À ÉVALUER :
{chr(10).join(f'- alt="{alt}" url="{url}"' for alt, url in images)}

Pour chaque image, réponds KEEP ou REMOVE en te basant sur :
— KEEP uniquement si l'URL et le texte alt indiquent CLAIREMENT que l'image illustre
  bien le sujet de la section où elle est placée (ex: image de vite.dev dans une section sur Vite 8 = KEEP)
— REMOVE si : l'image est hors-sujet, ambiguë, ou si l'URL ne donne pas confiance
  sur le contenu réel (ex: thumbnail horizontale d'un élément générique = REMOVE)
— En cas de doute : REMOVE

Réponds UNIQUEMENT avec une ligne par image : KEEP ou REMOVE, dans l'ordre.
Rien d'autre."""

    try:
        resp = client.messages.create(
            model="claude-haiku-4-5-20251001",
            max_tokens=200,
            messages=[{"role": "user", "content": prompt}],
        )
        decisions = [l.strip().upper() for l in resp.content[0].text.strip().split('\n') if l.strip()]
        removed = 0
        for (alt, url), decision in zip(images, decisions):
            if decision == "REMOVE":
                content = content.replace(f"![{alt}]({url})\n\n", "")
                content = content.replace(f"![{alt}]({url})", "")
                print(f"[img-review] REMOVED: {alt[:60]}")
                removed += 1
            else:
                print(f"[img-review] KEPT: {alt[:60]}")
        if removed:
            print(f"[img-review] {removed}/{len(images)} image(s) removed.")
        return content
    except Exception as e:
        print(f"[img-review] Failed: {e} — keeping all images", file=sys.stderr)
        return content


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
    images: list[dict] | None = None,
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

    images_context = ""
    if images:
        img_lines = [
            f'  - [{img["domain"]}] ![{img["article_title"][:60]}]({img["url"]})'
            for img in images
        ]
        images_context = f"""
IMAGES DISPONIBLES — à placer dans le texte là où elles illustrent le propos :
{chr(10).join(img_lines)}
Place chaque image après le premier paragraphe de la section correspondante, pas au début.
Format Markdown : ![Description courte](url)"""

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

    prompt = f"""Tu es un journaliste tech senior qui écrit la rétrospective hebdomadaire d'une veille technologique personnelle.

═══ IDENTITÉ ÉDITORIALE ═══

Ton style, c'est celui d'un développeur web senior — stack Rust, WebAssembly, sécurité web, Linux —
qui a des opinions claires, lit la recherche académique autant que les blogs d'ingénierie, et refuse
de reproduire les communiqués de presse déguisés en articles.

Tu prends position. Tu expliques POURQUOI quelque chose compte, pas juste CE QUE c'est.
Tu cites des chiffres précis, des noms de projets réels, des CVE IDs, des numéros de version.
Tu démystifies le hype et tu célèbres les vraies avancées, même discrètes.
Quand un sujet touche à la sécurité des lecteurs, tu le dis sans détour et tu donnes des actions concrètes.

Tu écris comme si tu envoyais une newsletter à des amis développeurs intelligents,
pas comme si tu remplissais un template de rédacteur SEO.

═══ CONTEXTE DE LA SEMAINE ═══

Période : {period_start} → {period_end} ({week_id})
Articles sélectionnés ({len(articles)} — choisis pour leur importance, pas pour leur volume) :
{context}
{prev_context}
{images_context}

═══ RÈGLES DE RÉDACTION — LIRE ATTENTIVEMENT ═══

1. COMMENCE PAR UNE ACCROCHE FORTE
   — Une scène concrète, un moment précis, un paradoxe, une question qui dérange.
   — PAS "Cette semaine a été riche en actualités". PAS de résumé en intro.
   — L'accroche doit donner envie de continuer à lire, pas résumer l'article.

2. CONSTRUIS UN FIL NARRATIF
   — Les sections ne sont pas des items de liste. Ce sont des actes d'un même récit.
   — Entre chaque section, il doit y avoir du LIANT : une phrase de transition qui explique
     pourquoi on passe de ce sujet à l'autre, ce qui les relie, ce que l'un éclaire de l'autre.
   — Exemple mauvais : [section CVE] --- [section Vite 8] sans lien.
   — Exemple bon : "Ce contexte de supply chain fragilisée rend d'autant plus pertinente
     la sortie de Vite 8 cette semaine — non pas comme une réponse directe, mais comme
     rappel que l'écosystème sait aussi, par moments, progresser."
   — Le lecteur doit sentir qu'il lit UN article, pas un agrégat de news.

3. HYPERLIENS OBLIGATOIRES EN MARKDOWN
   — Chaque outil, projet, organisation, publication mentionné pour la première fois
     doit être un lien cliquable vers sa source officielle ou principale.
   — Format : [Nom du projet](https://url-officielle.com)
   — Pour les articles sources fournis : utilise l'URL de l'article comme lien.
   — Minimum 6-8 hyperliens par article. Ce n'est pas une recommandation, c'est une règle.

4. IMAGES ET TABLEAUX
   — Si un article source a une image pertinente (OG image), intègre-la avec :
     ![Description courte](https://url-de-limage.jpg)
   — Place l'image APRÈS le premier paragraphe d'une section, pas au début.
   — Utilise un tableau Markdown quand tu compares des données (versions, benchmarks,
     délais, features). Maximum 1-2 tableaux par article.

5. OPINIONS ET POSITIONNEMENT
   — "C'est franchement impressionnant", "Soyons honnêtes, c'est discutable",
     "Ce que la plupart des articles ratent là-dedans, c'est que...",
     "Je ne suis pas sûr que l'industrie ait réalisé l'ampleur de..."
   — Prends des positions. L'article doit avoir une voix, pas une neutralité de rapport.

6. PAS DE DOUBLON TITRE
   — Ne commence PAS le contenu par "# Titre de l'article".
   — Commence directement par le premier paragraphe ou une citation forte.
   — Le titre est déjà affiché dans l'en-tête de la page.
{security_instruction}
{archi_instruction}

═══ STRUCTURE RECOMMANDÉE (FLEXIBLE) ═══

— 2-3 paragraphes d'accroche (pas de heading, juste du texte)
— ## Section 1 (sujet le plus fort ou le plus inattendu)
— [liant] ## Section 2
— [liant] ## Section 3 (etc.)
— ## Le fil de la semaine (ou titre équivalent — le sens de tout ça)
— [Si sécurité] ## Actions immédiates (avec code blocks)
— [Si architecture] section dédiée
— Paragraphe de clôture court, sans heading

═══ CONTRAINTES TECHNIQUES ═══

- Longueur : 1500-2500 mots par article.
- Markdown GFM. Les liens externes ouvrent dans un nouvel onglet (c'est géré par le frontend,
  écris juste des liens Markdown normaux).
- title_fr et title_en doivent être DIFFÉRENTS — deux angles éditoriaux sur la même semaine.
- content_fr en français, content_en en anglais.

═══ OUTPUT ═══

JSON pur — zéro texte avant ou après, zéro fence markdown :
{{
  "title_fr": "Titre magazine accrocheur (max 80 chars)",
  "title_en": "Punchy magazine title (max 80 chars, different angle from FR)",
  "content_fr": "[article FR — commence directement par le texte, PAS par # Titre]",
  "content_en": "[English article — starts directly with text, NOT with # Title]",
  "security_actions": {json.dumps(["Action CLI précise 1", "Action 2"] if security_present else [])},
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
            max_tokens=16000,
            messages=[{"role": "user", "content": prompt}],
        )
        text = response.content[0].text.strip()
        text = re.sub(r"^```(?:json)?\s*\n?", "", text)
        text = re.sub(r"\n?```\s*$", "", text)
        return json.loads(text)
    except Exception as e:
        print(f"[synthesis] generation failed: {e}", file=sys.stderr)
        return None


def generate_ai_synthesis(
    client: anthropic.Anthropic,
    articles: list[dict],
    period_start: str,
    period_end: str,
    previous_syntheses: list[dict],
    synth_id: str,
) -> dict | None:
    """
    Token-lean dedicated AI synthesis: ENGLISH ONLY, short, no illustrations.
    Goes straight to the point: where this week's AI news sits in the state of
    the art, short/medium/long-term implications, and concrete developer actions.
    One Sonnet call, no Haiku passes, no image fetching.
    """
    context = build_context(articles)

    prev_context = ""
    if previous_syntheses:
        prev_lines = [f'  - "{s.get("title_en","?")}"' for s in previous_syntheses[:3]]
        prev_context = (
            "\nPREVIOUS AI BRIEFS (do not repeat these angles):\n"
            + "\n".join(prev_lines)
        )

    prompt = f"""You are a senior engineer writing a sharp, no-fluff AI intelligence brief.
Access to frontier models is becoming geopolitically critical (export controls, model
access restrictions, regulation), so be concrete about what actually matters.

PERIOD: {period_start} → {period_end}
SELECTED AI ITEMS (labs, research, regulation, analysis):
{context}
{prev_context}

WRITE IN ENGLISH ONLY. Be concise and direct — 500-900 words total. No filler, no
"this week was busy" intros. Markdown GFM. Use Markdown hyperlinks to primary sources.
DO NOT start with a "# Title". NO images.

Cover, in this order, with short "## " sections:
1. What happened — the 2-4 items that genuinely matter (one tight paragraph or bullets).
2. ## Where it sits in the state of the art — how these moves fit (or break) the current
   AI frontier; what's genuinely new vs incremental.
3. ## Implications — short / medium / long term — be explicit about timeframe and about
   geopolitical/regulatory/access angles where relevant.
4. ## What to do as a developer — concrete, actionable steps (tools to try, deps to pin,
   things to watch, decisions to revisit). No vague "stay informed".

OUTPUT — pure JSON, nothing before or after, no markdown fence:
{{
  "title_en": "Punchy brief title (max 80 chars)",
  "content_en": "[English brief — starts directly with text, NOT with # Title]"
}}"""

    try:
        response = client.messages.create(
            model="claude-sonnet-4-6",
            max_tokens=4000,
            messages=[{"role": "user", "content": prompt}],
        )
        text = response.content[0].text.strip()
        text = re.sub(r"^```(?:json)?\s*\n?", "", text)
        text = re.sub(r"\n?```\s*$", "", text)
        return json.loads(text)
    except Exception as e:
        print(f"[ai-synthesis] generation failed: {e}", file=sys.stderr)
        return None


def ai_window(data: dict, now: datetime) -> tuple[datetime, datetime, str]:
    """
    Rolling window for the dedicated AI synthesis: from the period_end of the most
    recent AI synthesis (or now-7d if none) up to now. ID is date-based so the
    cadence (e.g. Mon + Thu) is free. Returns (start_dt, end_dt, synth_id).
    """
    prev_ends = [
        i.get("period_end", "")
        for i in data.get("items", [])
        if i.get("type") == "synthesis" and i.get("track") == "ai" and i.get("period_end")
    ]
    if prev_ends:
        last = max(prev_ends)  # "YYYY-MM-DD"
        y, m, d = (int(x) for x in last.split("-"))
        start_dt = datetime(y, m, d, 0, 0, 0, tzinfo=timezone.utc) + timedelta(days=1)
    else:
        start_dt = (now - timedelta(days=7)).replace(hour=0, minute=0, second=0, microsecond=0)
    end_dt = now
    synth_id = f"synthesis_ai_{now.strftime('%Y-%m-%d')}"
    return start_dt, end_dt, synth_id


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--week",  help="ISO week to synthesize, e.g. 2026-W22")
    parser.add_argument("--force", action="store_true", help="Overwrite existing synthesis for this week")
    parser.add_argument("--track", choices=["general", "ai"], default="general",
                        help="Which synthesis track to generate (default: general)")
    args = parser.parse_args()

    api_key = os.environ.get("ANTHROPIC_API_KEY")
    if not api_key:
        print("[synthesis] ANTHROPIC_API_KEY not set", file=sys.stderr)
        sys.exit(1)

    out_path = Path(os.environ.get("NEWS_JSON_PATH") or (Path(__file__).parent.parent / "public" / "news.json"))
    if not out_path.exists():
        print("[synthesis] news.json not found", file=sys.stderr)
        sys.exit(1)

    data = json.loads(out_path.read_text(encoding="utf-8"))
    now  = datetime.now(timezone.utc)
    track = args.track

    # Determine period + synthesis id (track-aware)
    if track == "ai":
        period_start_dt, period_end_dt, synthesis_id = ai_window(data, now)
    elif args.week:
        period_start_dt, period_end_dt = parse_week(args.week)
        synthesis_id = f"synthesis_{args.week.replace('-', '_')}"
    else:
        period_end_dt   = (now - timedelta(days=1)).replace(hour=23, minute=59, second=59, microsecond=0)
        period_start_dt = (now - timedelta(days=8)).replace(hour=0, minute=0, second=0, microsecond=0)
        synthesis_id    = isoweek_id(now)

    period_start = period_start_dt.strftime("%Y-%m-%d")
    period_end   = period_end_dt.strftime("%Y-%m-%d")
    print(f"[{track}] Period: {period_start} → {period_end}  [{synthesis_id}]")

    # Check if synthesis already exists
    existing_synth = next((i for i in data["items"] if i.get("id") == synthesis_id), None)
    if existing_synth and not args.force:
        print(f"[synthesis] {synthesis_id} already exists. Use --force to overwrite.")
        sys.exit(0)

    # Load previous syntheses for context (same track, exclude current id)
    previous_syntheses = [
        i for i in data["items"]
        if i.get("type") == "synthesis"
        and i.get("id") != synthesis_id
        and (i.get("track", "general") == track)
    ][:3]

    # Collect selected articles for this period, routed by track:
    #   ai      → only domain == "ai"
    #   general → everything except domain == "ai"
    def in_track(item: dict) -> bool:
        is_ai = item.get("domain") == "ai"
        return is_ai if track == "ai" else not is_ai

    candidate_articles = [
        item for item in data.get("items", [])
        if item.get("type", "article") == "article"
        and item.get("status") in ("selected", None)
        and in_track(item)
        and period_start_dt.isoformat() <= item.get("published_at", "") <= period_end_dt.isoformat()
    ]

    min_articles = 2 if track == "ai" else 3
    if len(candidate_articles) < min_articles:
        print(f"[synthesis] Only {len(candidate_articles)} {track} articles in window — need ≥ {min_articles}", file=sys.stderr)
        sys.exit(0)

    print(f"Synthesizing [{track}] from {len(candidate_articles)} selected articles:")
    for a in candidate_articles:
        score_str = f" [{a.get('score', '?'):.1f}]" if isinstance(a.get('score'), float) else ""
        print(f"  [{a.get('domain','?')}]{score_str} {a['title'][:70]}")

    print(f"\nPrevious syntheses context: {[s['id'] for s in previous_syntheses]}")

    client = anthropic.Anthropic(api_key=api_key)

    if track == "ai":
        # Token-lean path: English only, short, no images, no Haiku passes.
        print("\nGenerating AI brief (claude-sonnet-4-6, English only, ≤4000 tokens)…")
        ai = generate_ai_synthesis(
            client, candidate_articles, period_start, period_end,
            previous_syntheses, synthesis_id,
        )
        if not ai:
            print("[synthesis] AI generation failed", file=sys.stderr)
            sys.exit(1)
        title_en = ai.get("title_en", "AI Brief")
        synthesis = {
            "title_fr":         title_en,   # mirror — frontend renders EN for the AI track
            "title_en":         title_en,
            "content_fr":       "",
            "content_en":       ai.get("content_en", ""),
            "security_actions": [],
        }
        en_words = len(synthesis["content_en"].split())
        fr_words = 0
        print(f"[synthesis] Word count — EN: {en_words} (AI brief, EN only)")
        if en_words < 300:
            print(f"[synthesis] Warning: AI brief seems short ({en_words} words)")
        illustration = {}
        archi_visual = {}
    else:
        # Fetch OG images (up to 3, one per domain)
        print("\n[image] Fetching article images…")
        images = find_images(candidate_articles, max_images=3)
        print(f"[image] {len(images)} image(s) found.")

        print("\nGenerating synthesis (claude-sonnet-4-6, up to 16000 tokens)…")
        synthesis = generate_synthesis(
            client, candidate_articles, period_start, period_end,
            previous_syntheses, synthesis_id, images=images,
        )
        if not synthesis:
            print("[synthesis] Generation failed", file=sys.stderr)
            sys.exit(1)

        # Proofread French content
        print("\n[proofread] Fixing French style (Haiku)…")
        synthesis["content_fr"] = proofread_french(client, synthesis["content_fr"])
        print("[proofread] Done.")

        # Validate images in both languages
        context_summary = f"Article de veille tech semaine {synthesis_id} — sujets : {', '.join(a['title'][:40] for a in candidate_articles[:4])}"
        print("\n[img-review] Validating images…")
        synthesis["content_fr"] = validate_images(client, synthesis["content_fr"], context_summary)
        synthesis["content_en"] = validate_images(client, synthesis["content_en"], context_summary)

        # Word count check
        fr_words = len(synthesis.get("content_fr", "").split())
        en_words = len(synthesis.get("content_en", "").split())
        print(f"[synthesis] Word count — FR: {fr_words}, EN: {en_words}")
        if fr_words < 1000:
            print(f"[synthesis] Warning: FR article seems short ({fr_words} words)")

        illustration = images[0] if images else {}

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
        "track":             track,
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
