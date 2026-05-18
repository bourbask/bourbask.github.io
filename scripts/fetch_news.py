#!/usr/bin/env python3
"""
Fetches tech news from RSS feeds and HackerNews API.
Classifies items by keyword into categories.
Outputs public/news.json consumed by the Leptos frontend.
"""
import hashlib
import json
import os
import re
import sys
from datetime import datetime, timezone, timedelta
from pathlib import Path
from urllib.parse import urlparse

try:
    import feedparser
    import requests
except ImportError:
    print("Missing deps: pip install feedparser requests", file=sys.stderr)
    sys.exit(1)

# Patterns for low-quality posts (regex, case-insensitive)
NOISE_PATTERNS = [
    r"day \d+ of",
    r"#\d+daysof",
    r"week \d+ of",
    r"part \d+:",
    r"episode \d+",
    r"\[beginner\]",
    r"my first ",
    r"i built a ",
    r"i made a ",
    r"top \d+ ",
    r"how to get started",
    r"for beginners",
    r"my journey",
    r"hello world",
    r"tutorial for",
    r"step[\s-]by[\s-]step",
    r"complete guide",
    r"ultimate guide",
    r"you need to know",
    r"everything you",
    r"getting started with",
    r"introduction to",
    r"beginner.s guide",
    r"#showdev",
    r"show dev",
]
_NOISE_RE = re.compile("|".join(NOISE_PATTERNS), re.IGNORECASE)

# no_filter=True: content is inherently authoritative, skip noise check
SOURCES = [
    # Academic / Research
    {"name": "ArXiv CS.AI",       "url": "https://arxiv.org/rss/cs.AI",                                     "lang": "en", "no_filter": True, "max_items": 8},
    {"name": "ArXiv CS.SE",       "url": "https://arxiv.org/rss/cs.SE",                                     "lang": "en", "no_filter": True, "max_items": 8},
    {"name": "ArXiv CS.CR",       "url": "https://arxiv.org/rss/cs.CR",                                     "lang": "en", "no_filter": True, "max_items": 8},
    {"name": "ArXiv CS.PL",       "url": "https://arxiv.org/rss/cs.PL",                                     "lang": "en", "no_filter": True, "max_items": 8},
    {"name": "ACM Tech News",     "url": "https://technews.acm.org/feed.xml",                               "lang": "en"},
    {"name": "IEEE Spectrum",     "url": "https://spectrum.ieee.org/feeds/feed.rss",                        "lang": "en"},
    {"name": "Papers With Code",  "url": "https://paperswithcode.com/blog/feed/",                           "lang": "en", "no_filter": True},

    # Government / Standards
    {"name": "CISA Advisories",   "url": "https://www.cisa.gov/cybersecurity-advisories/feed",              "lang": "en", "no_filter": True},
    {"name": "NIST CSRC",         "url": "https://csrc.nist.gov/News/feed",                                 "lang": "en", "no_filter": True},
    {"name": "W3C Blog",          "url": "https://www.w3.org/blog/news/feed",                               "lang": "en", "no_filter": True},
    {"name": "IETF Blog",         "url": "https://www.ietf.org/blog/feed/",                                 "lang": "en", "no_filter": True},
    {"name": "ENISA",             "url": "https://www.enisa.europa.eu/news/enisa-news/rss",                 "lang": "en", "no_filter": True},

    # Official language / platform blogs
    {"name": "Rust Blog",         "url": "https://blog.rust-lang.org/feed.xml",                             "lang": "en", "no_filter": True},
    {"name": "Go Blog",           "url": "https://go.dev/blog/feed.atom",                                   "lang": "en", "no_filter": True},
    {"name": "Python Blog",       "url": "https://blog.python.org/feeds/posts/default",                     "lang": "en", "no_filter": True},
    {"name": "Node.js Blog",      "url": "https://nodejs.org/en/feed/blog.xml",                             "lang": "en", "no_filter": True},
    {"name": "Mozilla Hacks",     "url": "https://hacks.mozilla.org/feed/",                                 "lang": "en", "no_filter": True},
    {"name": "WebKit Blog",       "url": "https://webkit.org/feed/",                                        "lang": "en", "no_filter": True},
    {"name": "Chromium Blog",     "url": "https://blog.chromium.org/feeds/posts/default",                   "lang": "en", "no_filter": True},
    {"name": "GitHub Engineering","url": "https://github.blog/engineering/feed/",                           "lang": "en", "no_filter": True},
    {"name": "OpenSSF Blog",      "url": "https://openssf.org/feed/",                                      "lang": "en", "no_filter": True},
    {"name": "This Week in Rust", "url": "https://this-week-in-rust.org/atom.xml",                          "lang": "en", "no_filter": True},
    {"name": "Linux Foundation",  "url": "https://www.linuxfoundation.org/blog/feed/",                      "lang": "en", "no_filter": True},
    {"name": "Symfony Blog",      "url": "https://symfony.com/blog/feed/atom",                              "lang": "en", "no_filter": True},
    {"name": "PHP.net",           "url": "https://www.php.net/feed.atom",                                   "lang": "en", "no_filter": True},
    {"name": "React Blog",        "url": "https://react.dev/blog/rss.xml",                                  "lang": "en", "no_filter": True},

    # Quality aggregators
    {"name": "CNCF Blog",         "url": "https://www.cncf.io/feed/",                                      "lang": "en"},
    {"name": "LWN.net",           "url": "https://lwn.net/headlines/rss",                                   "lang": "en"},
    {"name": "InfoQ",             "url": "https://feed.infoq.com/",                                         "lang": "en"},
    {"name": "The Register Dev",  "url": "https://www.theregister.com/software/developer/headlines.atom",   "lang": "en"},
    {"name": "Lobste.rs",         "url": "https://lobste.rs/rss",                                           "lang": "en"},

    # Multilingual
    {"name": "LinuxFr.org",       "url": "https://linuxfr.org/news.atom",                                   "lang": "fr"},
    {"name": "Framablog",         "url": "https://framablog.org/feed/",                                     "lang": "fr"},
    {"name": "Golem.de",          "url": "https://rss.golem.de/rss.php?feed=RSS2.0",                        "lang": "de"},
    {"name": "Zenn",              "url": "https://zenn.dev/feed",                                           "lang": "ja"},
]

KEYWORDS: dict[str, list[str]] = {
    "urgent": [
        "cve-", "vulnerability", "vulnerabilit", "is deprecated", "deprecation notice",
        "breaking change", "security breach", "exploit", "critical patch",
        "zero-day", "0day", " rce ", "end of life", " eol ", "shutdown",
        "discontinu", "malware", "ransomware", "data breach", "supply chain attack",
        "patch tuesday", "security advisory", "security release",
    ],
    "good_news": [
        " released", "stable release", "generally available", "ga release",
        "open source", "performance boost", "faster than", "new feature",
        "announced", "available now", "ships with", "hits 1.0", "hits 2.0",
        "reaches stable", "open-sourced", "free tier",
    ],
    "future_watch": [
        " llm", " ai ", " ai-", "artificial intelligence", "machine learning",
        "roadmap", "2026", "2027", "emerging", "future of",
        "experimental", " alpha ", " beta ", " preview", "next generation",
        "webassembly", " wasm", "edge computing", "serverless",
        "bun ", "deno ", "htmx", "astro ", "leptos", "tauri",
        "web components", "signal", "resumability",
    ],
    "stack_alt": [
        " vs ", " versus ", "alternative to", "benchmark", "migration guide",
        "comparison", "instead of", "replacing", "migrate from",
        "nestjs", "next.js", "nuxt", "laravel", "django", "fastapi",
        "golang", " go ", "kotlin", "spring boot", "rails", "elixir",
    ],
}

MAX_HN_FETCH = 60   # fetch this many IDs to filter by score
MAX_HN_KEEP  = 20   # keep at most this many after filtering
MIN_HN_SCORE = 100
MAX_RSS_ITEMS = 10


def is_safe_url(url: str) -> bool:
    try:
        return urlparse(url).scheme.lower() in ("http", "https")
    except Exception:
        return False


def item_id(url: str) -> str:
    return hashlib.md5(url.encode(), usedforsecurity=False).hexdigest()[:12]


def classify(title: str, summary: str = "") -> list[str]:
    text = (title + " " + summary[:200]).lower()
    cats = [cat for cat, kws in KEYWORDS.items() if any(kw in text for kw in kws)]
    return cats or ["general"]


def parse_date(entry) -> str:
    for attr in ("published_parsed", "updated_parsed"):
        val = getattr(entry, attr, None)
        if val:
            try:
                return datetime(*val[:6], tzinfo=timezone.utc).isoformat()
            except Exception:
                pass
    return datetime.now(timezone.utc).isoformat()


def fetch_hn() -> list[dict]:
    items: list[dict] = []
    try:
        resp = requests.get(
            "https://hacker-news.firebaseio.com/v0/topstories.json",
            timeout=10,
        )
        resp.raise_for_status()
        story_ids = resp.json()[:MAX_HN_FETCH]
    except Exception as e:
        print(f"[HN] top stories fetch failed: {e}", file=sys.stderr)
        return items

    kept = 0
    for sid in story_ids:
        if kept >= MAX_HN_KEEP:
            break
        try:
            story = requests.get(
                f"https://hacker-news.firebaseio.com/v0/item/{sid}.json",
                timeout=5,
            ).json()
            if not story or story.get("type") != "story" or not story.get("url"):
                continue
            if story.get("score", 0) < MIN_HN_SCORE:
                continue
            url = story["url"]
            if not is_safe_url(url):
                continue
            title = story.get("title", "")
            items.append({
                "id": item_id(url),
                "title": title,
                "url": url,
                "source": "HackerNews",
                "categories": classify(title),
                "published_at": datetime.fromtimestamp(
                    story.get("time", 0), tz=timezone.utc
                ).isoformat(),
                "lang": "en",
            })
            kept += 1
        except Exception:
            continue

    return items


def fetch_rss(source: dict) -> list[dict]:
    items: list[dict] = []
    no_filter = source.get("no_filter", False)
    max_items = source.get("max_items", MAX_RSS_ITEMS)
    try:
        feed = feedparser.parse(source["url"])
        for entry in feed.entries[:max_items]:
            url = entry.get("link", "")
            title = entry.get("title", "")
            if not url or not title:
                continue
            if not is_safe_url(url):
                continue
            if not no_filter and _NOISE_RE.search(title):
                continue
            summary = entry.get("summary", "") or entry.get("description", "")
            summary_clean = re.sub(r"<[^>]+>", " ", summary) if summary else ""
            items.append({
                "id": item_id(url),
                "title": title,
                "url": url,
                "source": source["name"],
                "categories": classify(title, summary_clean),
                "published_at": parse_date(entry),
                "lang": source.get("lang", "en"),
            })
    except Exception as e:
        print(f"[RSS] {source['name']} failed: {e}", file=sys.stderr)
    return items


def generate_synthesis(items: list[dict]) -> dict | None:
    """Generate bilingual EN/FR synthesis via Claude API. Returns None if key absent or call fails."""
    api_key = os.environ.get("ANTHROPIC_API_KEY")
    if not api_key:
        return None
    try:
        import anthropic
    except ImportError:
        print("[synthesis] anthropic package not installed, skipping", file=sys.stderr)
        return None

    client = anthropic.Anthropic(api_key=api_key)

    by_cat: dict[str, list[str]] = {}
    for item in items[:80]:
        for cat in item["categories"]:
            by_cat.setdefault(cat, []).append(
                f"[{item['source']}][{item['lang']}] {item['title']}"
            )

    context_lines: list[str] = []
    for cat, titles in by_cat.items():
        context_lines.append(f"\n## {cat.upper()}")
        context_lines.extend(titles[:12])
    context = "\n".join(context_lines)

    prompt = f"""You are a critical tech analyst. Analyze these categorized tech articles from today and generate a bilingual (EN + FR) synthesis.

ARTICLES BY CATEGORY:
{context}

TOTAL: {len(items)} items collected from institutional, academic, and curated sources.

Generate a JSON object with this EXACT structure (pure JSON only, no markdown fences):
{{
  "en": {{
    "headline": "One punchy headline (the single most important story today)",
    "tldr": "2-3 sentence essential summary. Be specific: mention real names, version numbers, CVE IDs.",
    "sections": [
      {{"category": "urgent", "summary": "Concise summary for urgent/security items"}},
      {{"category": "good_news", "summary": "Concise summary for releases and good news"}},
      {{"category": "future_watch", "summary": "Concise summary for emerging tech items"}},
      {{"category": "stack_alt", "summary": "Concise summary for comparison/alternatives items"}}
    ],
    "key_takeaways": ["Actionable point 1", "Actionable point 2", "Actionable point 3"],
    "signal_vs_noise": "Honest 1-sentence assessment of today's signal quality"
  }},
  "fr": {{
    "headline": "...",
    "tldr": "...",
    "sections": [...],
    "key_takeaways": [...],
    "signal_vs_noise": "..."
  }}
}}

Rules:
- Only include sections for categories that have substantive content (omit empty ones)
- Be critical: distinguish real releases/discoveries from marketing/hype
- key_takeaways must be actionable (what a developer should know or do today)
- Respond ONLY with valid JSON, no text before or after"""

    try:
        response = client.messages.create(
            model="claude-haiku-4-5-20251001",
            max_tokens=2500,
            messages=[{"role": "user", "content": prompt}],
        )
        text = response.content[0].text.strip()
        # Strip markdown code fences if Claude adds them
        text = re.sub(r"^```(?:json)?\s*\n?", "", text)
        text = re.sub(r"\n?```\s*$", "", text)
        return json.loads(text)
    except Exception as e:
        print(f"[synthesis] generation failed: {e}", file=sys.stderr)
        return None


def main() -> None:
    out_path = Path(__file__).parent.parent / "public" / "news.json"
    all_items: list[dict] = []
    seen: set[str] = set()

    def add(item: dict) -> None:
        if item["id"] not in seen:
            seen.add(item["id"])
            all_items.append(item)

    # Load existing items: keep synthesis cards (no expiry) + articles within 14-day window
    cutoff = (datetime.now(timezone.utc) - timedelta(days=14)).isoformat()
    if out_path.exists():
        try:
            existing = json.loads(out_path.read_text(encoding="utf-8"))
            for item in existing.get("items", []):
                if item.get("type") == "synthesis":
                    if item["id"] not in seen:
                        seen.add(item["id"])
                        all_items.append(item)
                elif item.get("published_at", "") >= cutoff:
                    if item["id"] not in seen:
                        seen.add(item["id"])
                        all_items.append(item)
        except Exception as e:
            print(f"[load] Failed to load existing news.json: {e}", file=sys.stderr)

    print("Fetching HackerNews…")
    for item in fetch_hn():
        add(item)

    for source in SOURCES:
        print(f"Fetching {source['name']}…")
        for item in fetch_rss(source):
            add(item)

    all_items.sort(key=lambda x: x.get("published_at", ""), reverse=True)

    print(f"Generating synthesis ({len(all_items)} items)…")
    synthesis = generate_synthesis(all_items)
    if synthesis:
        print("[synthesis] Generated successfully.")
    else:
        print("[synthesis] Skipped (no ANTHROPIC_API_KEY or generation failed).")

    output = {
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "period": datetime.now(timezone.utc).strftime("%Y-%m-%d"),
        "count": len(all_items),
        "synthesis": synthesis,
        "items": all_items,
    }

    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(json.dumps(output, ensure_ascii=False, indent=2), encoding="utf-8")
    print(f"Done: {len(all_items)} items → {out_path}")


if __name__ == "__main__":
    main()
