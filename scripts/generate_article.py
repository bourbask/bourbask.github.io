#!/usr/bin/env python3
"""
Open source tool article generator.
Fetches GitHub data, generates bilingual article with Claude Sonnet,
inserts Rust code into mod.rs, and creates a PR.

Usage:
  python scripts/generate_article.py --github dani-garcia/vaultwarden
  python scripts/generate_article.py --github dani-garcia/vaultwarden --dry-run
  python scripts/generate_article.py --id vaultwarden   # from tools_candidates.json
"""
import argparse
import base64
import json
import os
import re
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path


def _load_dotenv() -> None:
    """Load .env from project root into os.environ (no extra deps needed)."""
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

GITHUB_HEADERS = {"Accept": "application/vnd.github.v3+json"}

# Article sections for open source tool discovery articles
SECTIONS_STRUCTURE = [
    ("le-probleme",     "Le problème que ça résout",            "The Problem It Solves"),
    ("ce-que-cest",     "Ce que c'est",                         "What It Is"),
    ("pourquoi-lessayer", "Pourquoi l'essayer",                 "Why You Should Try It"),
    ("en-pratique",     "En pratique — setup et usage",         "In Practice — Setup & Usage"),
    ("limites",         "Limites honnêtes",                     "Honest Limitations"),
    ("pour-qui",        "Pour qui",                             "Who It's For"),
    ("verdict",         "Verdict",                              "Verdict"),
]

CATEGORY_MAP = {
    "devops": "devops",
    "securite": "security",
    "productivite": "devtools",
    "self-hosting": "self-hosting",
    "monitoring": "devops",
    "data": "data",
    "iot": "hardware",
    "impression-3d": "hardware",
    "admin-quotidien": "project",
    "jeux-video": "hardware",
    "ia-local": "devops",
}


def fetch_repo_data(github_slug: str) -> dict:
    """Fetch repo metadata from GitHub API."""
    resp = requests.get(
        f"https://api.github.com/repos/{github_slug}",
        headers=GITHUB_HEADERS, timeout=10,
    )
    resp.raise_for_status()
    return resp.json()


def fetch_readme(github_slug: str) -> str:
    """Fetch and decode README content."""
    resp = requests.get(
        f"https://api.github.com/repos/{github_slug}/readme",
        headers=GITHUB_HEADERS, timeout=10,
    )
    if resp.status_code != 200:
        return ""
    data = resp.json()
    content = data.get("content", "")
    try:
        decoded = base64.b64decode(content).decode("utf-8", errors="replace")
        # Trim to ~6000 chars to keep the prompt manageable
        return decoded[:6000]
    except Exception:
        return ""


def fetch_latest_release(github_slug: str) -> str:
    """Get the latest release tag."""
    try:
        resp = requests.get(
            f"https://api.github.com/repos/{github_slug}/releases/latest",
            headers=GITHUB_HEADERS, timeout=8,
        )
        if resp.status_code == 200:
            return resp.json().get("tag_name", "")
    except Exception:
        pass
    return ""


def generate_article_content(
    client: anthropic.Anthropic,
    repo: dict,
    readme: str,
    latest_release: str,
    category: str,
) -> dict | None:
    """Generate bilingual article JSON from repo data."""

    github_slug = f"{repo['owner']['login']}/{repo['name']}"
    stars = repo.get("stargazers_count", 0)
    description = repo.get("description") or ""
    topics = ", ".join(repo.get("topics", [])[:10])
    language = repo.get("language") or "N/A"
    license_name = (repo.get("license") or {}).get("name", "unknown")
    homepage = repo.get("homepage") or ""

    sections_fr = "\n".join(
        f'  - Section "{s[0]}": titre FR = "{s[1]}"' for s in SECTIONS_STRUCTURE
    )
    sections_en = "\n".join(
        f'  - Section "{s[0]}": EN title = "{s[2]}"' for s in SECTIONS_STRUCTURE
    )

    prompt = f"""Tu rédiges un article de découverte d'outil open source pour un blog tech personnel.

OUTIL : {repo['name']}
GitHub : https://github.com/{github_slug}
Description : {description}
Stars : {stars:,} ⭐
Langage principal : {language}
Licence : {license_name}
Dernière release : {latest_release or 'N/A'}
Topics : {topics}
Site officiel : {homepage or 'N/A'}

README (extrait) :
---
{readme[:4000]}
---

PROFIL DU LECTEUR :
Développeur fullstack senior (Rust, TypeScript, PHP/Symfony, Docker, Linux).
Aime : self-hosting, sécurité, productivité dev, IA locale, IoT, impression 3D.
Valeurs : souveraineté numérique, open source, frugalité technique.
Il veut savoir si cet outil vaut la peine d'être essayé — pas un tutoriel complet.

TON ÉDITORIAL :
- Direct, technique, sans bullshit corporate
- "Je te vends cet outil" — enthousiaste mais honnête sur les limites
- Exemples concrets tirés du README/documentation
- Commandes réelles issues de la vraie doc
- Prend position clairement dans le verdict

SECTIONS À RÉDIGER (FR + EN) :
{sections_fr}

{sections_en}

CONTRAINTES :
- Chaque section : 150-300 mots, HTML propre (<p>, <ul>, <li>, <pre><code>, <strong>)
- Callouts : <div class="article-callout tip/info/warning"><span class="callout-icon">💡/ℹ️/⚠️</span><div class="callout-content"><p>...</p></div></div>
- Code : <pre><code>commande ici</code></pre>
- Liens : <a href="URL" target="_blank" rel="noopener noreferrer">texte</a>
- NE PAS commencer par # titre (c'est géré par la structure)
- Le contenu HTML de chaque section ne doit PAS contenir la séquence ###" (pour compatibilité Rust raw strings)
- read_time : estimation en minutes (5-15 min)
- tags : 4-8 tags pertinents en kebab-case

OUTPUT — JSON pur, aucun texte avant ou après, aucune fence markdown :
{{
  "id": "{repo['name'].lower().replace(' ', '-').replace('_', '-')}",
  "title_fr": "Titre accrocheur en français (max 70 chars)",
  "title_en": "Punchy English title (max 70 chars)",
  "subtitle_fr": "Sous-titre descriptif en français (max 120 chars)",
  "subtitle_en": "Descriptive subtitle in English (max 120 chars)",
  "description_fr": "Description SEO en français (max 160 chars)",
  "description_en": "SEO description in English (max 160 chars)",
  "tldr_fr": "Résumé en 2-3 phrases techniques, concises",
  "tldr_en": "2-3 sentence technical summary",
  "read_time": 8,
  "tags": ["tag1", "tag2", "tag3"],
  "category": "{category}",
  "sections": [
    {{
      "id": "le-probleme",
      "title_fr": "Le problème que ça résout",
      "title_en": "The Problem It Solves",
      "content_fr": "<p>Contenu HTML en français...</p>",
      "content_en": "<p>HTML content in English...</p>"
    }}
  ]
}}"""

    try:
        resp = client.messages.create(
            model="claude-sonnet-4-6",
            max_tokens=8000,
            messages=[{"role": "user", "content": prompt}],
        )
        text = resp.content[0].text.strip()
        text = re.sub(r"^```(?:json)?\s*\n?", "", text)
        text = re.sub(r"\n?```\s*$", "", text)
        return json.loads(text)
    except Exception as e:
        print(f"[generate] Failed: {e}", file=sys.stderr)
        return None


def escape_rust_string(s: str) -> str:
    """Escape a string for use in a Rust raw string r###'...'###."""
    # Replace any ###" sequence to avoid breaking the raw string delimiter
    return s.replace('###"', '##\\"')


def article_to_rust(article: dict, github_slug: str, stars: int) -> str:
    """Convert article JSON to Rust struct code."""
    aid = article["id"]
    now = datetime.now(timezone.utc).strftime("%Y-%m-%d")

    tags = ", ".join(f'"{t}".to_string()' for t in article.get("tags", []))

    sections_fr = ""
    sections_en = ""
    for sec in article.get("sections", []):
        sec_id = sec["id"]
        content_fr = escape_rust_string(sec["content_fr"])
        content_en = escape_rust_string(sec["content_en"])
        title_fr = sec["title_fr"].replace('"', '\\"')
        title_en = sec["title_en"].replace('"', '\\"')

        sections_fr += f"""                        ArticleSection {{
                            id: "{sec_id}".to_string(),
                            title: "{title_fr}".to_string(),
                            content: r###"{content_fr}"###.to_string(),
                        }},
"""
        sections_en += f"""                        ArticleSection {{
                            id: "{sec_id}".to_string(),
                            title: "{title_en}".to_string(),
                            content: r###"{content_en}"###.to_string(),
                        }},
"""

    read_time = article.get("read_time", 8)
    category = CATEGORY_MAP.get(article.get("category", "devtools"), "devtools")
    tldr_fr = escape_rust_string(article.get("tldr_fr", ""))
    tldr_en = escape_rust_string(article.get("tldr_en", ""))

    return f"""        // =========================================================
        // ARTICLE: {aid}
        // GitHub: https://github.com/{github_slug} ({stars:,} ⭐)
        // Generated: {now}
        // =========================================================
        Article {{
            meta: ArticleMeta {{
                id: "{aid}".to_string(),
                title: {{
                    let mut m = HashMap::new();
                    m.insert("fr".to_string(), "{article['title_fr'].replace('"', chr(92) + '"')}".to_string());
                    m.insert("en".to_string(), "{article['title_en'].replace('"', chr(92) + '"')}".to_string());
                    m
                }},
                subtitle: {{
                    let mut m = HashMap::new();
                    m.insert("fr".to_string(), "{article['subtitle_fr'].replace('"', chr(92) + '"')}".to_string());
                    m.insert("en".to_string(), "{article['subtitle_en'].replace('"', chr(92) + '"')}".to_string());
                    m
                }},
                description: {{
                    let mut m = HashMap::new();
                    m.insert("fr".to_string(), "{article['description_fr'].replace('"', chr(92) + '"')}".to_string());
                    m.insert("en".to_string(), "{article['description_en'].replace('"', chr(92) + '"')}".to_string());
                    m
                }},
                date: "{now}".to_string(),
                read_time: {read_time},
                tags: vec![{tags}],
                category: "{category}".to_string(),
                featured: false,
                image: "/images/placeholder.jpg".to_string(),
                status: ArticleStatus::Published,
            }},
            content: {{
                let mut m = HashMap::new();

                m.insert("fr".to_string(), ArticleContent {{
                    tldr: r###"{tldr_fr}"###.to_string(),
                    sections: vec![
{sections_fr}                    ],
                }});

                m.insert("en".to_string(), ArticleContent {{
                    tldr: r###"{tldr_en}"###.to_string(),
                    sections: vec![
{sections_en}                    ],
                }});

                m
            }},
        }},
"""


def insert_article_into_mod(rust_code: str, mod_path: Path) -> None:
    """Insert article Rust code at the start of get_all_articles() vec![]."""
    content = mod_path.read_text(encoding="utf-8")
    marker = "    vec![\n"
    idx = content.find(marker)
    if idx == -1:
        raise ValueError("Could not find 'vec![' insertion point in mod.rs")
    insert_pos = idx + len(marker)
    new_content = content[:insert_pos] + rust_code + content[insert_pos:]
    mod_path.write_text(new_content, encoding="utf-8")


def update_generated_log(github_slug: str, article_id: str, generated_path: Path) -> None:
    """Track generated articles to avoid duplicates."""
    data = {"articles": []}
    if generated_path.exists():
        try:
            data = json.loads(generated_path.read_text())
        except Exception:
            pass
    data["articles"].append({
        "id": article_id,
        "github": github_slug,
        "generated_at": datetime.now(timezone.utc).isoformat(),
    })
    generated_path.write_text(json.dumps(data, ensure_ascii=False, indent=2), encoding="utf-8")


def create_pr(branch: str, article_id: str, title_fr: str) -> str | None:
    """Create a GitHub PR for the new article."""
    try:
        result = subprocess.run(
            ["gh", "pr", "create",
             "--title", f"content: article — {article_id}",
             "--body", f"## Nouvel article\n\n**{title_fr}**\n\nGénéré automatiquement via `generate_article.py`.\n\n> Relire avant de merger.",
             "--head", branch,
             "--base", "main"],
            capture_output=True, text=True, check=True,
        )
        return result.stdout.strip()
    except subprocess.CalledProcessError as e:
        print(f"[pr] Failed: {e.stderr}", file=sys.stderr)
        return None


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--github", help="GitHub slug: owner/repo")
    parser.add_argument("--id", help="Tool ID from tools_candidates.json")
    parser.add_argument("--dry-run", action="store_true", help="Print Rust code, don't write")
    args = parser.parse_args()

    if not args.github and not args.id:
        parser.error("Provide --github owner/repo or --id tool-id")

    api_key = os.environ.get("ANTHROPIC_API_KEY")
    if not api_key:
        print("[generate] ANTHROPIC_API_KEY not set", file=sys.stderr)
        sys.exit(1)

    root = Path(__file__).parent.parent
    mod_path = root / "src" / "data" / "articles" / "mod.rs"
    generated_path = root / "public" / "tools_generated.json"
    candidates_path = root / "public" / "tools_candidates.json"

    # Resolve github slug
    github_slug = args.github
    category = "productivite"

    if args.id and not github_slug:
        if candidates_path.exists():
            data = json.loads(candidates_path.read_text())
            for c in data.get("candidates", []):
                if c.get("id") == args.id or c.get("github", "").split("/")[-1] == args.id:
                    github_slug = c["github"]
                    category = c.get("category", category)
                    break
        if not github_slug:
            print(f"[generate] Tool '{args.id}' not found in candidates", file=sys.stderr)
            sys.exit(1)

    print(f"Fetching GitHub data for {github_slug}…")
    try:
        repo = fetch_repo_data(github_slug)
    except Exception as e:
        print(f"[generate] GitHub fetch failed: {e}", file=sys.stderr)
        sys.exit(1)

    readme = fetch_readme(github_slug)
    latest_release = fetch_latest_release(github_slug)
    stars = repo.get("stargazers_count", 0)

    print(f"  {repo['name']} — {stars:,} ⭐ — {repo.get('description','')[:60]}")
    print(f"  README: {len(readme)} chars, release: {latest_release or 'none'}")

    client = anthropic.Anthropic(api_key=api_key)
    print("\nGenerating article (claude-sonnet-4-6)…")
    article = generate_article_content(client, repo, readme, latest_release, category)

    if not article:
        print("[generate] Article generation failed", file=sys.stderr)
        sys.exit(1)

    print(f"  Title FR: {article.get('title_fr', '')}")
    print(f"  Sections: {len(article.get('sections', []))}")

    rust_code = article_to_rust(article, github_slug, stars)

    if args.dry_run:
        print("\n" + "="*60)
        print(rust_code[:2000] + ("…" if len(rust_code) > 2000 else ""))
        print("[dry-run] Not written.")
        return

    article_id = article["id"]
    branch = f"content/article-{article_id}"

    # Git: create branch
    subprocess.run(["git", "checkout", "-b", branch], check=True)

    # Insert into mod.rs
    print(f"\nInserting into mod.rs…")
    insert_article_into_mod(rust_code, mod_path)

    # Update generated log
    update_generated_log(github_slug, article_id, generated_path)

    # Commit
    subprocess.run(["git", "add", str(mod_path), str(generated_path)], check=True)
    subprocess.run(
        ["git", "commit", "-m", f"content: add article — {article_id} ({github_slug})"],
        check=True,
    )
    subprocess.run(["git", "push", "-u", "origin", branch], check=True)

    # PR
    print("\nCreating PR…")
    pr_url = create_pr(branch, article_id, article.get("title_fr", article_id))
    if pr_url:
        print(f"PR: {pr_url}")
    else:
        print(f"Branch pushed: {branch} — create PR manually on GitHub")

    # Back to main
    subprocess.run(["git", "checkout", "main"], check=True)

    print(f"\nDone: article '{article_id}' generated and PR created.")


if __name__ == "__main__":
    main()
