#!/usr/bin/env python3
"""
Fetches tech news from RSS feeds and HackerNews API.
Classifies items by keyword into categories.
Outputs public/news.json consumed by the Leptos frontend.
"""
import hashlib
import json
import re
import sys
from datetime import datetime, timezone
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
    r"day \d+ of",          # #100DaysOfCode
    r"#\d+daysof",
    r"week \d+ of",
    r"part \d+:",
    r"episode \d+",
    r"\[beginner\]",
    r"my first ",
    r"i built a ",
    r"i made a ",
]
_NOISE_RE = re.compile("|".join(NOISE_PATTERNS), re.IGNORECASE)

SOURCES = [
    {"name": "dev.to — Symfony",  "type": "rss", "url": "https://dev.to/feed/tag/symfony",  "lang": "en"},
    {"name": "dev.to — React",    "type": "rss", "url": "https://dev.to/feed/tag/react",    "lang": "en"},
    {"name": "dev.to — Rust",     "type": "rss", "url": "https://dev.to/feed/tag/rust",     "lang": "en"},
    {"name": "dev.to — PHP",      "type": "rss", "url": "https://dev.to/feed/tag/php",      "lang": "en"},
    {"name": "dev.to — DevOps",   "type": "rss", "url": "https://dev.to/feed/tag/devops",   "lang": "en"},
    {"name": "dev.to — AI",       "type": "rss", "url": "https://dev.to/feed/tag/ai",       "lang": "en"},
    {"name": "Lobste.rs",         "type": "rss", "url": "https://lobste.rs/rss",             "lang": "en"},
    {"name": "Symfony Blog",      "type": "rss", "url": "https://symfony.com/blog/feed/atom","lang": "en"},
    {"name": "Rust Blog",         "type": "rss", "url": "https://blog.rust-lang.org/feed.xml","lang": "en"},
    {"name": "React Blog",        "type": "rss", "url": "https://react.dev/blog/rss.xml",   "lang": "en"},
    {"name": "PHP.net",           "type": "rss", "url": "https://www.php.net/feed.atom",    "lang": "en"},
    {"name": "CNCF Blog",         "type": "rss", "url": "https://www.cncf.io/feed/",        "lang": "en"},
    {"name": "r/programming",     "type": "rss", "url": "https://www.reddit.com/r/programming/.rss", "lang": "en"},
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

MAX_HN_STORIES = 40
MAX_RSS_ITEMS = 10


def is_safe_url(url: str) -> bool:
    try:
        return urlparse(url).scheme.lower() in ("http", "https")
    except Exception:
        return False


def item_id(url: str) -> str:
    return hashlib.md5(url.encode(), usedforsecurity=False).hexdigest()[:12]


def classify(title: str, summary: str = "") -> list[str]:
    # Use full title but cap summary to reduce false positives from long articles
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
        story_ids = resp.json()[:MAX_HN_STORIES]
    except Exception as e:
        print(f"[HN] top stories fetch failed: {e}", file=sys.stderr)
        return items

    for sid in story_ids:
        try:
            story = requests.get(
                f"https://hacker-news.firebaseio.com/v0/item/{sid}.json",
                timeout=5,
            ).json()
            if not story or story.get("type") != "story" or not story.get("url"):
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
        except Exception:
            continue

    return items


def fetch_rss(source: dict) -> list[dict]:
    items: list[dict] = []
    try:
        feed = feedparser.parse(source["url"])
        for entry in feed.entries[:MAX_RSS_ITEMS]:
            url = entry.get("link", "")
            title = entry.get("title", "")
            if not url or not title:
                continue
            if not is_safe_url(url):
                continue
            if _NOISE_RE.search(title):
                continue
            summary = entry.get("summary", "") or entry.get("description", "")
            # Strip HTML tags from summary for keyword matching
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


def main() -> None:
    all_items: list[dict] = []
    seen: set[str] = set()

    def add(item: dict) -> None:
        if item["id"] not in seen:
            seen.add(item["id"])
            all_items.append(item)

    print("Fetching HackerNews…")
    for item in fetch_hn():
        add(item)

    for source in SOURCES:
        print(f"Fetching {source['name']}…")
        for item in fetch_rss(source):
            add(item)

    all_items.sort(key=lambda x: x["published_at"], reverse=True)

    output = {
        "generated_at": datetime.now(timezone.utc).isoformat(),
        "count": len(all_items),
        "items": all_items,
    }

    out_path = Path(__file__).parent.parent / "public" / "news.json"
    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(json.dumps(output, ensure_ascii=False, indent=2), encoding="utf-8")
    print(f"Done: {len(all_items)} items → {out_path}")


if __name__ == "__main__":
    main()
