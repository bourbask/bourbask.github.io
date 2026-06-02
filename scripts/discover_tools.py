#!/usr/bin/env python3
"""
Weekly open source tool discovery pipeline.
Fetches candidates from GitHub Search + HN Algolia, scores them with Claude Haiku,
and writes public/tools_candidates.json.

Usage:
  python scripts/discover_tools.py
  python scripts/discover_tools.py --dry-run     # print results, no write
  python scripts/discover_tools.py --category ia-local
"""
import argparse
import json
import os
import re
import sys
import time
from datetime import datetime, timezone, timedelta
from pathlib import Path

def _load_dotenv() -> None:
    env_path = Path(__file__).parent.parent / ".env"
    if not env_path.exists():
        return
    for line in env_path.read_text(encoding="utf-8").splitlines():
        line = line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, _, val = line.partition("=")
        key = key.strip()
        val = val.strip().strip('"').strip("'")
        if key and key not in os.environ:
            os.environ[key] = val

_load_dotenv()

try:
    import requests
    import anthropic
except ImportError:
    print("Missing deps: pip install requests anthropic", file=sys.stderr)
    sys.exit(1)

# ── User profile — used by Claude for scoring relevance ────────────────────────
USER_PROFILE = """
Développeur fullstack senior, 27 ans, basé en Bretagne.
Stack principale : Rust, TypeScript/React, PHP/Symfony, Docker, Linux (Manjaro daily driver).
Centres d'intérêt tech : self-hosting, sécurité web, DevOps, monitoring, impression 3D,
IoT/domotique, IA locale, productivité terminal, jeux vidéo (émulation, serveurs dédiés).
Hors tech : randonnée côtière bretonne, claviers mécaniques custom, plantes.
Valeurs : souveraineté numérique, open source, frugalité technique, privacy.
Il cherche des OUTILS À ESSAYER pour améliorer son process perso et pro —
pas des tutos, pas des articles marketing : des projets open source concrets,
maintenus, qui résolvent un vrai problème.
"""

# ── Categories and associated GitHub topics ────────────────────────────────────
CATEGORIES: dict[str, dict] = {
    "devops": {
        "label": "DevOps & Infrastructure",
        "topics": ["ansible", "terraform", "docker", "kubernetes", "infrastructure-as-code", "ci-cd", "deployment"],
        "hn_keywords": ["devops", "infrastructure", "kubernetes", "terraform", "ansible"],
    },
    "securite": {
        "label": "Sécurité",
        "topics": ["security", "secrets-management", "authentication", "vulnerability-scanner", "sast", "pentest", "audit"],
        "hn_keywords": ["security", "secrets", "authentication", "zero trust", "pentest"],
    },
    "productivite": {
        "label": "Productivité dev",
        "topics": ["cli-tool", "developer-tools", "automation", "terminal", "shell", "productivity"],
        "hn_keywords": ["cli tool", "developer productivity", "terminal", "automation"],
    },
    "self-hosting": {
        "label": "Self-hosting",
        "topics": ["self-hosted", "homelab", "selfhosted", "docker-compose", "vps"],
        "hn_keywords": ["self-hosted", "homelab", "self hosting"],
    },
    "monitoring": {
        "label": "Monitoring & Observabilité",
        "topics": ["monitoring", "observability", "prometheus", "grafana", "alerting", "logging", "metrics"],
        "hn_keywords": ["monitoring", "observability", "alerting", "uptime"],
    },
    "data": {
        "label": "Data",
        "topics": ["database", "analytics", "etl", "data-pipeline", "data-visualization", "sql"],
        "hn_keywords": ["database", "analytics", "data pipeline", "visualization"],
    },
    "iot": {
        "label": "IoT & Domotique",
        "topics": ["iot", "home-automation", "mqtt", "homeassistant", "smart-home", "zigbee", "esphome"],
        "hn_keywords": ["home automation", "iot", "smart home", "mqtt", "zigbee"],
    },
    "impression-3d": {
        "label": "Impression 3D",
        "topics": ["3d-printing", "slicer", "klipper", "marlin", "octoprint", "3d-printer"],
        "hn_keywords": ["3d printing", "klipper", "slicer", "octoprint"],
    },
    "admin-quotidien": {
        "label": "Administration du quotidien",
        "topics": ["budgeting", "finance", "home-management", "accounting", "invoice", "expense-tracker"],
        "hn_keywords": ["budget", "finance", "expense tracking", "self-hosted accounting"],
    },
    "jeux-video": {
        "label": "Jeux vidéo",
        "topics": ["game-server", "emulator", "game-development", "modding", "retro-gaming"],
        "hn_keywords": ["game server", "emulator", "retro gaming", "game dev open source"],
    },
    "ia-local": {
        "label": "IA locale",
        "topics": ["llm", "local-llm", "ollama", "inference", "rag", "ai-agent", "language-model"],
        "hn_keywords": ["local llm", "ollama", "self-hosted ai", "inference", "rag"],
    },
}

MIN_STARS = 200
MAX_RESULTS_PER_TOPIC = 5
MAX_HN_PER_CATEGORY = 3
HEADERS = {"Accept": "application/vnd.github.v3+json"}


def github_search(topic: str, min_stars: int = MIN_STARS) -> list[dict]:
    """Search GitHub repos by topic."""
    pushed_since = (datetime.now() - timedelta(days=365)).strftime("%Y-%m-%d")
    query = f"topic:{topic} stars:>{min_stars} pushed:>{pushed_since}"
    try:
        resp = requests.get(
            "https://api.github.com/search/repositories",
            params={"q": query, "sort": "stars", "order": "desc", "per_page": MAX_RESULTS_PER_TOPIC},
            headers=HEADERS,
            timeout=10,
        )
        if resp.status_code == 403:
            print(f"[github] Rate limited on topic={topic}", file=sys.stderr)
            return []
        resp.raise_for_status()
        items = resp.json().get("items", [])
        return [
            {
                "github": f"{r['owner']['login']}/{r['name']}",
                "name": r["name"],
                "description": r.get("description") or "",
                "stars": r["stargazers_count"],
                "topics": r.get("topics", []),
                "language": r.get("language") or "",
                "license": (r.get("license") or {}).get("spdx_id", "unknown"),
                "homepage": r.get("homepage") or "",
                "last_push": r.get("pushed_at", ""),
                "url": r["html_url"],
                "source": "github_search",
            }
            for r in items
            if not r.get("archived", False)
        ]
    except Exception as e:
        print(f"[github] search topic={topic}: {e}", file=sys.stderr)
        return []


def hn_search(keyword: str, min_points: int = 150) -> list[dict]:
    """Search HN Show HN stories via Algolia."""
    try:
        resp = requests.get(
            "https://hn.algolia.com/api/v1/search",
            params={
                "query": f"Show HN {keyword}",
                "tags": "story",
                "numericFilters": f"points>{min_points}",
                "hitsPerPage": MAX_HN_PER_CATEGORY,
            },
            timeout=10,
        )
        resp.raise_for_status()
        hits = resp.json().get("hits", [])
        results = []
        for h in hits:
            url = h.get("url", "")
            # Only keep GitHub links
            m = re.match(r"https://github\.com/([^/]+/[^/?\s]+)", url)
            if not m:
                continue
            results.append({
                "github": m.group(1).strip("/"),
                "name": h.get("title", "").replace("Show HN: ", ""),
                "description": h.get("title", ""),
                "stars": 0,  # will be enriched
                "topics": [],
                "language": "",
                "license": "",
                "homepage": "",
                "last_push": "",
                "url": url,
                "source": "hackernews",
                "hn_points": h.get("points", 0),
            })
        return results
    except Exception as e:
        print(f"[hn] search '{keyword}': {e}", file=sys.stderr)
        return []


def enrich_github(item: dict) -> dict:
    """Fetch repo details for items missing stars/topics (e.g. from HN)."""
    if item.get("stars", 0) > 0:
        return item
    try:
        resp = requests.get(
            f"https://api.github.com/repos/{item['github']}",
            headers=HEADERS,
            timeout=8,
        )
        if resp.status_code != 200:
            return item
        r = resp.json()
        item["stars"] = r.get("stargazers_count", 0)
        item["topics"] = r.get("topics", [])
        item["language"] = r.get("language") or ""
        item["license"] = (r.get("license") or {}).get("spdx_id", "")
        item["last_push"] = r.get("pushed_at", "")
        item["description"] = r.get("description") or item["description"]
    except Exception:
        pass
    return item


def already_generated(github: str, generated_path: Path) -> bool:
    if not generated_path.exists():
        return False
    try:
        data = json.loads(generated_path.read_text())
        return any(a.get("github") == github for a in data.get("articles", []))
    except Exception:
        return False


def score_candidates(
    client: anthropic.Anthropic,
    candidates: list[dict],
    category_key: str,
    category_label: str,
) -> list[dict]:
    """Score a batch of candidates with Claude Haiku."""
    if not candidates:
        return []

    lines = []
    for i, c in enumerate(candidates):
        lines.append(
            f"{i+1}. [{c['github']}] ★{c['stars']} — {c['description'][:120]} "
            f"(lang:{c['language']}, topics:{','.join(c['topics'][:5])})"
        )

    prompt = f"""Tu évalues des projets open source pour la catégorie "{category_label}".

PROFIL DU LECTEUR :
{USER_PROFILE}

PROJETS À ÉVALUER ({len(candidates)}) :
{chr(10).join(lines)}

Pour chaque projet, donne un score de 0 à 10 selon :
- Pertinence pour le profil du lecteur (40%)
- Valeur ajoutée réelle pour son quotidien dev/perso (40%)
- Originalité (pas trop connu, pas évident) (20%)

Réponds UNIQUEMENT avec un JSON array — aucun texte avant ou après :
[{{"rank": 1, "score": 8.5, "reason": "max 15 mots"}}]

Même nombre d'éléments que de projets, dans le même ordre."""

    try:
        resp = client.messages.create(
            model="claude-haiku-4-5-20251001",
            max_tokens=800,
            messages=[{"role": "user", "content": prompt}],
        )
        text = resp.content[0].text.strip()
        text = re.sub(r"^```(?:json)?\s*", "", text)
        text = re.sub(r"\s*```$", "", text)
        scores = json.loads(text)
        for i, s in enumerate(scores[:len(candidates)]):
            candidates[i]["score"] = float(s.get("score", 0))
            candidates[i]["score_reason"] = s.get("reason", "")
            candidates[i]["category"] = category_key
        return sorted(candidates, key=lambda x: x.get("score", 0), reverse=True)
    except Exception as e:
        print(f"[score] {category_label}: {e}", file=sys.stderr)
        for c in candidates:
            c["score"] = 5.0
            c["score_reason"] = "fallback"
            c["category"] = category_key
        return candidates


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--dry-run", action="store_true")
    parser.add_argument("--category", help="Run only for this category key")
    args = parser.parse_args()

    api_key = os.environ.get("ANTHROPIC_API_KEY")
    if not api_key:
        print("[discover] ANTHROPIC_API_KEY not set — scoring disabled", file=sys.stderr)

    root = Path(__file__).parent.parent
    out_path = root / "public" / "tools_candidates.json"
    generated_path = root / "public" / "tools_generated.json"

    categories = (
        {args.category: CATEGORIES[args.category]}
        if args.category and args.category in CATEGORIES
        else CATEGORIES
    )

    client = anthropic.Anthropic(api_key=api_key) if api_key else None

    all_candidates: list[dict] = []
    seen_github: set[str] = set()

    for cat_key, cat_info in categories.items():
        print(f"\n── {cat_info['label']} ──────────────────────────────────")
        batch: list[dict] = []

        # GitHub Search
        for topic in cat_info["topics"][:4]:
            results = github_search(topic)
            for r in results:
                g = r["github"]
                if g not in seen_github and not already_generated(g, generated_path):
                    seen_github.add(g)
                    batch.append(r)
            time.sleep(0.5)  # GitHub rate limit courtesy

        # HN
        for kw in cat_info["hn_keywords"][:2]:
            for r in hn_search(kw):
                g = r["github"]
                if g not in seen_github and not already_generated(g, generated_path):
                    seen_github.add(g)
                    batch.append(enrich_github(r))
            time.sleep(0.3)

        print(f"  {len(batch)} candidates fetched")

        # Score
        if client and batch:
            batch = score_candidates(client, batch, cat_key, cat_info["label"])
        else:
            for c in batch:
                c["category"] = cat_key
                c["score"] = 5.0

        # Keep top 5 per category
        top = batch[:5]
        for c in top:
            print(f"  [{c['score']:.1f}] {c['github']} — {c.get('score_reason','')[:50]}")
        all_candidates.extend(top)

    # Sort globally by score
    all_candidates.sort(key=lambda x: x.get("score", 0), reverse=True)

    output = {
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "count": len(all_candidates),
        "candidates": all_candidates,
    }

    if args.dry_run:
        print(f"\n[dry-run] {len(all_candidates)} candidates — not written")
        for c in all_candidates[:10]:
            print(f"  [{c['score']:.1f}][{c['category']}] {c['github']}")
        return

    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(json.dumps(output, ensure_ascii=False, indent=2), encoding="utf-8")
    print(f"\nDone: {len(all_candidates)} candidates → {out_path}")


if __name__ == "__main__":
    main()
