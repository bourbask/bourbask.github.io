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
import time
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


def call_claude_json(client, log_prefix: str, attempts: int = 3, **create_kwargs) -> dict | None:
    """
    Call Claude and parse its response as JSON, retrying on JSONDecodeError.
    The model occasionally emits malformed JSON (unterminated string, stray
    control character) — almost always a transient escaping glitch, not a
    systematic prompt issue, so a plain retry recovers it instead of losing
    the whole week's synthesis.
    """
    last_err: Exception | None = None
    for attempt in range(1, attempts + 1):
        try:
            response = client.messages.create(**create_kwargs)
            text = response.content[0].text.strip()
            text = re.sub(r"^```(?:json)?\s*\n?", "", text)
            text = re.sub(r"\n?```\s*$", "", text)
            return json.loads(text)
        except json.JSONDecodeError as e:
            last_err = e
            print(f"[{log_prefix}] JSON parse failed (attempt {attempt}/{attempts}): {e}", file=sys.stderr)
            if attempt < attempts:
                time.sleep(2 * attempt)
        except Exception as e:
            print(f"[{log_prefix}] generation failed: {e}", file=sys.stderr)
            return None
    print(f"[{log_prefix}] generation failed after {attempts} attempts: {last_err}", file=sys.stderr)
    return None


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
            headers={"User-Agent": "bourbask.github.io/veille-bot"},
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


# Substrings that flag a generic / non-illustrative OG image (mail envelope, logo,
# share-card default, RSS icon…). These are the junk heroes we kept getting.
GENERIC_IMAGE_PATTERNS = (
    "envelope", "/email", "e-mail", "mailicon", "newsletter",
    "logo", "default", "placeholder", "/icon", "sprite", "avatar",
    "share-", "og-default", "feed-icon", "/rss",
)


def is_generic_image(url: str) -> bool:
    u = url.lower()
    return any(p in u for p in GENERIC_IMAGE_PATTERNS)


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
        if img and is_generic_image(img):
            print(f"[image] Skipped generic OG image: {img[:70]}")
            continue
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


def vet_illustration(client: anthropic.Anthropic, illustration: dict, context_summary: str) -> dict | None:
    """
    Relevance gate for the hero illustration (general track only). The hero was never
    checked before — a generic OG (envelope/logo) slipped straight onto the card.
    Returns the illustration if it clearly illustrates the article, else None.
    """
    if not illustration or not illustration.get("url"):
        return None
    prompt = f"""Image d'illustration en tête d'un article tech.
CONTEXTE : {context_summary}
Image : url="{illustration.get('url','')}" légende="{illustration.get('caption','')}"

Réponds KEEP si l'image illustre clairement un vrai sujet de l'article (capture produit,
schéma, photo ou visuel pertinent). Réponds DROP si elle est générique ou hors-sujet
(enveloppe, logo, icône, bannière vide, image de partage par défaut). En cas de doute : DROP.
Un seul mot : KEEP ou DROP."""
    try:
        resp = client.messages.create(
            model="claude-haiku-4-5-20251001",
            max_tokens=5,
            messages=[{"role": "user", "content": prompt}],
        )
        verdict = resp.content[0].text.strip().upper()
        if verdict.startswith("KEEP"):
            return illustration
        print(f"[illustration] Dropped (not relevant): {illustration.get('url','')[:70]}")
        return None
    except Exception as e:
        print(f"[illustration] vet failed: {e} — keeping", file=sys.stderr)
        return illustration


def build_context(items: list[dict]) -> str:
    by_domain: dict[str, list[str]] = {}
    for item in items:
        domain = item.get("domain", item.get("categories", ["general"])[0])
        score  = item.get("score")
        score_str = f" [score:{score:.1f}]" if score else ""
        # srcref:<id> is the link token — the model links a source via [text](srcref:ID),
        # never its raw URL (which it mangles). resolve_source_links() swaps in the exact URL.
        entry  = f'srcref:{item.get("id","?")} — [{item.get("source","?")}][{item.get("lang","en")}]{score_str} {item["title"]}'
        by_domain.setdefault(domain, []).append(entry)

    lines = []
    for domain, titles in sorted(by_domain.items()):
        lines.append(f"\n## {domain.upper().replace('_', ' ')}")
        lines.extend(titles)
    return "\n".join(lines)


def resolve_source_links(content: str, id_to_url: dict[str, str]) -> str:
    """
    Replace the srcref:<id> link tokens emitted by the model with the EXACT feed URLs.
    We never let the model type a source article's URL — it hallucinates/mangles them
    (404s). Handles both link form `](srcref:ID)` and any stray bare `srcref:ID`.
    """
    if not content:
        return content

    def link_repl(m: "re.Match") -> str:
        url = id_to_url.get(m.group(1))
        return f"]({url})" if url else m.group(0)

    content = re.sub(r"\]\(\s*srcref:([A-Za-z0-9_]+)\s*\)", link_repl, content)

    def bare_repl(m: "re.Match") -> str:
        url = id_to_url.get(m.group(1))
        return url if url else m.group(0)

    return re.sub(r"\bsrcref:([A-Za-z0-9_]+)\b", bare_repl, content)


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

    prompt = f"""Tu écris la rétrospective hebdomadaire d'une veille tech personnelle. Deux versions : une française, une anglaise.

═══ QUI ÉCRIT ═══

Un développeur web senior — Rust, WebAssembly, sécurité web, Linux. Opinions tranchées,
lit la recherche académique autant que les blogs d'ingénierie, allergique aux communiqués
de presse déguisés en articles. Tu écris à des amis devs intelligents, pas à un moteur SEO.

Tu expliques POURQUOI un truc compte, pas seulement CE QUE c'est. Chiffres précis, noms de
projets réels, CVE IDs, numéros de version. Tu dégonfles le hype et tu célèbres les vraies
avancées, même discrètes.

═══ LA VOIX — le point le plus important, c'est ici que la plupart des synthèses sonnent « IA » ═══

Objectif : que ça sonne ÉCRIT PAR UN HUMAIN. Pas de neutralité de rapport.

— Rythme varié : alterne phrases courtes qui claquent et phrases plus longues et denses.
  Une phrase de trois mots après deux phrases chargées, ça réveille le lecteur.
— Adresse-toi à lui quand c'est utile ("si tu es en prod avec une intégration calibrée pour
  la v2, prépare-toi à re-tester").
— Registre parlé mais précis : des tournures vivantes ("X remet le couvert", "ça change pas
  mal la donne", "noir sur blanc") sans familiarité forcée ni vannes gratuites.
— Verdicts assumés, asides secs, une pointe d'humour quand le sujet s'y prête. Une vraie
  opinion vaut mieux qu'un "il est intéressant de noter".
— Le concret avant l'abstrait : une scène, un détail, un chiffre qui parle, plutôt qu'une
  généralité. Si tu compares, compare avec des nombres réels.

INTERDITS — ce sont les signatures d'un texte IA, bannis-les :
— "En résumé", "En somme", "Force est de constater", "Il convient de noter", "Dans un monde où",
  "À l'heure où", "plongeons dans", "décryptage", "tour d'horizon", "incontournable", "fascinant".
— Les transitions mécaniques empilées ("Par ailleurs", "De plus", "En outre", "Enfin").
— Le faux équilibre permanent ("d'une part… d'autre part") et les tricolons réflexes.
— TOUTE phrase qui laisse transparaître la consigne. N'écris JAMAIS un truc du genre
  "les sujets de la semaine sont liés par X parce que Y" — ça expose le prompt en clair.
  Si un fil existe, montre-le par une observation concrète ; s'il n'existe pas, ne le force pas.

VERSION ANGLAISE : même exigence, registre d'un bon journalisme tech US — direct, opinioné,
factuel, qui tranche sans se cacher. Ce n'est PAS une traduction littérale du français :
c'est le même sujet réécrit, avec son propre angle et son propre titre.

Inspire-toi de l'humanité du bon journalisme — SANS imiter la signature d'un auteur précis.
La voix reste la tienne : le dev senior décrit plus haut.

═══ CONTEXTE DE LA SEMAINE ═══

Période : {period_start} → {period_end} ({week_id})
Articles sélectionnés ({len(articles)} — choisis pour leur importance, pas pour leur volume) :
{context}
{prev_context}
{images_context}

═══ MÉCANIQUE DU TEXTE ═══

— Accroche : une scène, un paradoxe, un chiffre qui dérange. Jamais "Cette semaine a été riche".
— Fil narratif : les sections sont les actes d'un même récit, pas une liste à puces. Les
  transitions doivent dire quelque chose (ce qu'un sujet éclaire de l'autre), pas juste enchaîner.
— Hyperliens Markdown : minimum 6-8 liens. Pour lier un ARTICLE FOURNI ci-dessus, écris le lien
  avec SON TOKEN comme cible : `[ton texte](srcref:ID)`. N'écris JAMAIS l'URL brute d'un article
  fourni — tu la déformes systématiquement ; le token sera remplacé par l'URL exacte. Pour un
  autre site (projet/orga officiel), ne mets un lien que si tu es CERTAIN de l'URL ; en cas de
  doute, cite sans lien plutôt que d'inventer une URL.
— Images : si une OG image pertinente est fournie, place-la APRÈS le 1er paragraphe de sa section
  (format ![desc courte](url)). Tableau Markdown uniquement pour comparer des données (1-2 max).
— Ne commence PAS par "# Titre" (il est déjà affiché). Commence par le texte.
{security_instruction}
{archi_instruction}

═══ STRUCTURE = INTENTION, PAS SCRIPT ═══

Ce qui suit est une orientation, pas un gabarit à remplir. Juge toi-même ce qui colle aux
articles réels — fusionne, réordonne, saute une étape si besoin.

— 2-3 paragraphes d'accroche (sans heading)
— quelques sections ## (le sujet le plus fort en premier)
— une clôture SEULEMENT si un vrai fil conducteur relie les sujets. S'il existe, révèle-le par
  une formule journalistique trouvée, pas par un bilan scolaire. S'il n'existe pas, termine sur
  le sujet le plus marquant — surtout pas de conclusion-bilan artificielle ni de méta-résumé.
— [Si sécurité] ## Actions immédiates (avec code blocks)
— [Si architecture] section dédiée

═══ CONTRAINTES TECHNIQUES ═══

- Longueur : 1500-2500 mots, mais vise la DENSITÉ, pas le remplissage. Coupe toute phrase qui
  n'apporte ni info, ni opinion, ni rythme. Mieux vaut 1600 mots tendus que 2400 dilués.
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

    return call_claude_json(
        client,
        "synthesis",
        model="claude-sonnet-4-6",
        max_tokens=16000,
        messages=[{"role": "user", "content": prompt}],
    )


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

    prompt = f"""You write a sharp weekly AI intelligence brief.

WHO WRITES: a senior engineer — Rust, WebAssembly, web security, Linux background — who
reads the research as much as the press releases, holds opinions, and deflates hype.
Access to frontier models is turning geopolitically critical (export controls, model-access
restrictions, regulation), so be concrete about what actually moves the needle.

PERIOD: {period_start} → {period_end}
SELECTED AI ITEMS (labs, research, regulation, analysis):
{context}
{prev_context}

═══ VOICE — this is where AI-generated writing gives itself away ═══

Make it read like a HUMAN wrote it, not a model.
— Vary the rhythm: short sentences that land, next to longer dense ones.
— Take a stance. A real verdict beats "it is worth noting that".
— Concrete over abstract: a number, a name, a specific consequence — not a generality.
— Speak to the reader when it helps ("if you pinned to that API last quarter, re-test now").
— Register of good US tech journalism: direct, opinionated, factual, unafraid to call it.
  Draw on that humanity WITHOUT imitating any specific writer's signature. The voice stays
  yours — the senior engineer above.

BANNED (the fingerprints of machine prose): "In summary", "In conclusion", "It's worth noting",
"In a world where", "delve into", "dive in", "landscape", "game-changer", "fascinating",
"In today's fast-paced". No stacked mechanical transitions ("Moreover", "Furthermore",
"Additionally"). No reflexive rule-of-three. And NEVER write a sentence that exposes the brief's
own scaffolding (e.g. "this week's items are connected by X because Y") — if a thread exists,
show it through a concrete observation; if it doesn't, don't manufacture one.

═══ SHAPE — intent, not a template ═══

English only. Tight: 500-900 words. No "this week was busy" intro. Markdown GFM, no "# Title"
line, no images. To link a SELECTED item above, use ITS TOKEN as the target: `[text](srcref:ID)`
— never its raw URL (it gets mangled); the token is swapped for the exact URL. For any other
site, link only if you're sure of the URL, otherwise mention it without a link. The four beats
below are the spine — merge, reorder or drop headings to fit the actual items, don't fill a form:

— What genuinely happened (the 2-4 items that matter).
— Where it sits in the state of the art — what's actually new vs incremental.
— Implications, short / medium / long term — name the timeframe; geopolitical/regulatory/access
  angle where it's real.
— What to do as a developer — concrete moves (tools to try, deps to pin, decisions to revisit),
  never a vague "stay informed".

OUTPUT — pure JSON, nothing before or after, no markdown fence:
{{
  "title_en": "Punchy brief title (max 80 chars)",
  "content_en": "[English brief — starts directly with text, NOT with # Title]"
}}"""

    return call_claude_json(
        client,
        "ai-synthesis",
        model="claude-sonnet-4-6",
        max_tokens=4000,
        messages=[{"role": "user", "content": prompt}],
    )


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


# Hard caps on how many articles feed a single synthesis. Daily scoring keeps ~5
# "selected" per day, so a 7-day window accumulates ~35 — feeding them all blows
# past the model's output budget and truncates the JSON. Capping to the top few
# bounds tokens DURABLY and keeps each synthesis focused on what matters.
MAX_SYNTHESIS_ARTICLES = 8   # general track
MAX_AI_ARTICLES        = 6   # dedicated AI track
MAX_PER_DOMAIN         = 3   # keep narrative variety in the general track


def cap_articles(articles: list[dict], limit: int, per_domain: int) -> list[dict]:
    """Top-N articles by score, with a soft per-domain cap to preserve variety."""
    ranked = sorted(articles, key=lambda a: a.get("score") or 0.0, reverse=True)
    out: list[dict] = []
    seen: dict[str, int] = {}
    for art in ranked:
        d = art.get("domain", "?")
        if seen.get(d, 0) >= per_domain:
            continue
        out.append(art)
        seen[d] = seen.get(d, 0) + 1
        if len(out) >= limit:
            break
    return out


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

    # Cap the article count so the prompt + generated JSON stay within the model's
    # output budget (otherwise the response truncates and json.loads fails).
    if track == "ai":
        limit, per_domain = MAX_AI_ARTICLES, MAX_AI_ARTICLES  # all one domain → no per-domain cap
    else:
        limit, per_domain = MAX_SYNTHESIS_ARTICLES, MAX_PER_DOMAIN
    if len(candidate_articles) > limit:
        dropped = len(candidate_articles) - limit
        candidate_articles = cap_articles(candidate_articles, limit, per_domain)
        print(f"[synthesis] Capped to top {limit} by score ({dropped} lower-scored dropped — bounds tokens).")

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

        # Hero illustration — gate it for relevance (was previously unchecked → junk heroes)
        illustration = images[0] if images else {}
        if illustration:
            illustration = vet_illustration(client, illustration, context_summary) or {}

        # Architecture visual
        archi_info   = synthesis.get("architecture_info", {})
        archi_visual = {}
        if archi_info.get("present") and archi_info.get("search_query"):
            print(f"[wikipedia] Fetching: {archi_info['search_query']}")
            archi_visual = fetch_wikipedia_info(archi_info["search_query"])
            if archi_visual.get("image_url"):
                print(f"[wikipedia] Image: {archi_visual['image_url'][:60]}…")

    # Swap srcref:<id> tokens for the EXACT feed URLs (the model never types source URLs).
    id_to_url = {a["id"]: a.get("url", "") for a in candidate_articles if a.get("url")}
    synthesis["content_fr"] = resolve_source_links(synthesis.get("content_fr", ""), id_to_url)
    synthesis["content_en"] = resolve_source_links(synthesis.get("content_en", ""), id_to_url)

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
