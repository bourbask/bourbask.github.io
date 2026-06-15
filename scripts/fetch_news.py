#!/usr/bin/env python3
"""
Fetches tech news from RSS feeds and HackerNews API.
Classifies items by keyword and domain.
Outputs public/news.json — NO AI synthesis (moved to score_articles.py).
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
    r"i learned ",
    r"how i ",
    r"my experience",
]
_NOISE_RE = re.compile("|".join(NOISE_PATTERNS), re.IGNORECASE)

# Domains: each source belongs to exactly one domain.
# score_articles.py runs a per-domain competition to select the best articles.
SOURCES = [
    # ── DOMAIN: dev_stack ────────────────────────────────────────────────────
    {"name": "Rust Blog",          "url": "https://blog.rust-lang.org/feed.xml",                             "lang": "en", "domain": "dev_stack", "no_filter": True},
    {"name": "This Week in Rust",  "url": "https://this-week-in-rust.org/atom.xml",                          "lang": "en", "domain": "dev_stack", "no_filter": True},
    {"name": "Go Blog",            "url": "https://go.dev/blog/feed.atom",                                   "lang": "en", "domain": "dev_stack", "no_filter": True},
    {"name": "Mozilla Hacks",      "url": "https://hacks.mozilla.org/feed/",                                 "lang": "en", "domain": "dev_stack", "no_filter": True},
    {"name": "WebKit Blog",        "url": "https://webkit.org/feed/",                                        "lang": "en", "domain": "dev_stack", "no_filter": True},
    {"name": "GitHub Engineering", "url": "https://github.blog/engineering/feed/",                           "lang": "en", "domain": "dev_stack", "no_filter": True},
    {"name": "Linux Foundation",   "url": "https://www.linuxfoundation.org/blog/feed/",                      "lang": "en", "domain": "dev_stack"},
    {"name": "LWN.net",            "url": "https://lwn.net/headlines/rss",                                   "lang": "en", "domain": "dev_stack"},
    {"name": "Lobste.rs",          "url": "https://lobste.rs/rss",                                           "lang": "en", "domain": "dev_stack"},
    {"name": "LinuxFr.org",        "url": "https://linuxfr.org/news.atom",                                   "lang": "fr", "domain": "dev_stack"},
    {"name": "ArXiv CS.PL",        "url": "https://arxiv.org/rss/cs.PL",                                     "lang": "en", "domain": "dev_stack", "no_filter": True, "max_items": 8},

    # ── DOMAIN: ai ───────────────────────────────────────────────────────────
    # Dedicated AI watch — quality sources only. Excluded from the general synthesis.
    # NOTE: feeds verified reachable via scripts/check_feeds.py. Anthropic, Meta AI and
    # Mistral publish no public RSS, so they are intentionally absent (no dead feeds).
    # ⟶ labos officiels
    {"name": "OpenAI",             "url": "https://openai.com/news/rss.xml",                                 "lang": "en", "domain": "ai", "no_filter": True, "max_items": 8},
    {"name": "Google DeepMind",    "url": "https://deepmind.google/blog/rss.xml",                            "lang": "en", "domain": "ai", "no_filter": True, "max_items": 8},
    {"name": "Hugging Face",       "url": "https://huggingface.co/blog/feed.xml",                            "lang": "en", "domain": "ai", "no_filter": True, "max_items": 6},
    {"name": "Microsoft Research", "url": "https://www.microsoft.com/en-us/research/feed/",                  "lang": "en", "domain": "ai", "no_filter": True, "max_items": 6},
    # ⟶ recherche académique
    {"name": "ArXiv CS.AI",        "url": "https://arxiv.org/rss/cs.AI",                                     "lang": "en", "domain": "ai", "no_filter": True, "max_items": 10},
    {"name": "ArXiv CS.LG",        "url": "https://arxiv.org/rss/cs.LG",                                     "lang": "en", "domain": "ai", "no_filter": True, "max_items": 8},
    {"name": "ArXiv CS.CL",        "url": "https://arxiv.org/rss/cs.CL",                                     "lang": "en", "domain": "ai", "no_filter": True, "max_items": 8},
    {"name": "Google Research",    "url": "https://research.google/blog/rss/",                               "lang": "en", "domain": "ai", "no_filter": True, "max_items": 6},
    {"name": "BAIR Blog",          "url": "https://bair.berkeley.edu/blog/feed.xml",                         "lang": "en", "domain": "ai", "no_filter": True, "max_items": 6},
    # ⟶ réglementaire / légal / politique (cs.CY = AI governance/ethics papers)
    {"name": "ArXiv CS.CY",        "url": "https://arxiv.org/rss/cs.CY",                                     "lang": "en", "domain": "ai", "no_filter": True, "max_items": 6},
    {"name": "EU Digital Strategy","url": "https://digital-strategy.ec.europa.eu/en/rss.xml",               "lang": "en", "domain": "ai", "no_filter": True, "max_items": 8},
    {"name": "NIST News",          "url": "https://www.nist.gov/news-events/news/rss.xml",                  "lang": "en", "domain": "ai", "no_filter": True, "max_items": 8},
    # ⟶ analyse / sécurité IA
    {"name": "Import AI",          "url": "https://importai.substack.com/feed",                              "lang": "en", "domain": "ai", "no_filter": True, "max_items": 4},
    {"name": "The Gradient",       "url": "https://thegradient.pub/rss/",                                    "lang": "en", "domain": "ai", "no_filter": True, "max_items": 4},
    {"name": "MIT Tech Review AI", "url": "https://www.technologyreview.com/topic/artificial-intelligence/feed", "lang": "en", "domain": "ai", "no_filter": True, "max_items": 6},
    {"name": "Ars Technica AI",    "url": "https://arstechnica.com/ai/feed/",                                "lang": "en", "domain": "ai", "no_filter": True, "max_items": 6},

    # ── DOMAIN: business_market (broad tech/market signal) ───────────────────
    {"name": "IEEE Spectrum",      "url": "https://spectrum.ieee.org/feeds/feed.rss",                        "lang": "en", "domain": "business_market"},
    {"name": "ACM Tech News",      "url": "https://technews.acm.org/feed.xml",                               "lang": "en", "domain": "business_market"},

    # ── DOMAIN: security ─────────────────────────────────────────────────────
    {"name": "CISA Advisories",    "url": "https://www.cisa.gov/cybersecurity-advisories/feed",              "lang": "en", "domain": "security", "no_filter": True},
    {"name": "NIST CSRC",          "url": "https://csrc.nist.gov/News/feed",                                 "lang": "en", "domain": "security", "no_filter": True},
    {"name": "ENISA",              "url": "https://www.enisa.europa.eu/news/enisa-news/rss",                 "lang": "en", "domain": "security", "no_filter": True},
    {"name": "OpenSSF Blog",       "url": "https://openssf.org/feed/",                                      "lang": "en", "domain": "security", "no_filter": True},
    {"name": "Krebs on Security",  "url": "https://krebsonsecurity.com/feed/",                               "lang": "en", "domain": "security", "no_filter": True},
    {"name": "Schneier on Security","url": "https://www.schneier.com/blog/atom.xml",                         "lang": "en", "domain": "security", "no_filter": True},
    {"name": "PortSwigger Blog",   "url": "https://portswigger.net/blog/rss",                                "lang": "en", "domain": "security", "no_filter": True},
    {"name": "SANS ISC",           "url": "https://isc.sans.edu/rssfeed_full.xml",                           "lang": "en", "domain": "security", "no_filter": True, "max_items": 5},

    # ── DOMAIN: health_science ───────────────────────────────────────────────
    {"name": "ArXiv CS.HC",        "url": "https://arxiv.org/rss/cs.HC",                                     "lang": "en", "domain": "health_science", "no_filter": True, "max_items": 8},
    {"name": "ArXiv q-bio",        "url": "https://arxiv.org/rss/q-bio",                                     "lang": "en", "domain": "health_science", "no_filter": True, "max_items": 8},
    {"name": "eLife Sciences",     "url": "https://elifesciences.org/rss/alerts/general.xml",                "lang": "en", "domain": "health_science", "no_filter": True, "max_items": 8},
    {"name": "PLOS ONE",           "url": "https://journals.plos.org/plosone/feed/atom",                     "lang": "en", "domain": "health_science", "no_filter": True, "max_items": 8},

    # ── DOMAIN: business_market ──────────────────────────────────────────────
    {"name": "The Register",       "url": "https://www.theregister.com/software/developer/headlines.atom",   "lang": "en", "domain": "business_market"},
    {"name": "InfoQ",              "url": "https://feed.infoq.com/",                                         "lang": "en", "domain": "business_market"},
    {"name": "CNCF Blog",          "url": "https://www.cncf.io/feed/",                                       "lang": "en", "domain": "business_market"},
    {"name": "W3C Blog",           "url": "https://www.w3.org/blog/news/feed",                               "lang": "en", "domain": "business_market", "no_filter": True},
    {"name": "IETF Blog",          "url": "https://www.ietf.org/blog/feed/",                                 "lang": "en", "domain": "business_market", "no_filter": True},
    # HackerNews is handled separately → added to business_market with higher score threshold

    # ── DOMAIN: architecture ─────────────────────────────────────────────────
    {"name": "Dezeen Sustainable", "url": "https://www.dezeen.com/tag/sustainable-architecture/feed/",       "lang": "en", "domain": "architecture"},
    {"name": "Low-tech Magazine",  "url": "https://solar.lowtechmagazine.com/index.xml",                     "lang": "en", "domain": "architecture", "no_filter": True},
    {"name": "ArchDaily",          "url": "https://www.archdaily.com/feed/rss",                              "lang": "en", "domain": "architecture", "max_items": 8},
    {"name": "TreeHugger",         "url": "https://www.treehugger.com/latest/rss",                           "lang": "en", "domain": "architecture", "max_items": 8},
    {"name": "Resilient Design",   "url": "https://www.resilientdesign.org/feed/",                           "lang": "en", "domain": "architecture", "no_filter": True},
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
        "web components", "signal", "resumability", "quantum",
    ],
    "stack_alt": [
        " vs ", " versus ", "alternative to", "benchmark", "migration guide",
        "comparison", "instead of", "replacing", "migrate from",
        "nestjs", "next.js", "nuxt", "laravel", "django", "fastapi",
        "golang", " go ", "kotlin", "spring boot", "rails", "elixir",
    ],
}

MAX_HN_FETCH   = 80
MAX_HN_KEEP    = 12
MIN_HN_SCORE   = 150  # raised from 100 — only significant stories
MAX_RSS_ITEMS  = 10

# Retention by status (days)
RETENTION = {
    "synthesis":      None,   # keep forever
    "selected":       21,     # 3 weeks — synthesis window
    "archived":       4,      # short — already lost the competition
    "raw":            2,      # if not scored within 2 days, expire
    None:             14,     # legacy articles without status
}


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
            if _NOISE_RE.search(title):
                continue
            items.append({
                "id":           item_id(url),
                "title":        title,
                "url":          url,
                "source":       "HackerNews",
                "domain":       "business_market",
                "categories":   classify(title),
                "published_at": datetime.fromtimestamp(
                    story.get("time", 0), tz=timezone.utc
                ).isoformat(),
                "lang":         "en",
                "status":       "raw",
            })
            kept += 1
        except Exception:
            continue

    return items


def fetch_rss(source: dict) -> list[dict]:
    items: list[dict] = []
    no_filter = source.get("no_filter", False)
    max_items = source.get("max_items", MAX_RSS_ITEMS)
    domain    = source.get("domain", "dev_stack")
    try:
        feed = feedparser.parse(source["url"])
        for entry in feed.entries[:max_items]:
            url   = entry.get("link", "")
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
                "id":           item_id(url),
                "title":        title,
                "url":          url,
                "source":       source["name"],
                "domain":       domain,
                "categories":   classify(title, summary_clean),
                "published_at": parse_date(entry),
                "lang":         source.get("lang", "en"),
                "status":       "raw",
            })
    except Exception as e:
        print(f"[RSS] {source['name']} failed: {e}", file=sys.stderr)
    return items


def is_expired(item: dict, now_iso: str) -> bool:
    status    = item.get("status") or item.get("type")
    days      = RETENTION.get(status, RETENTION[None])
    if days is None:
        return False
    cutoff = (datetime.now(timezone.utc) - timedelta(days=days)).isoformat()
    pub = item.get("published_at", "")
    return pub < cutoff


def main() -> None:
    out_path   = Path(os.environ.get("NEWS_JSON_PATH") or (Path(__file__).parent.parent / "public" / "news.json"))
    fetched_at = datetime.now(timezone.utc).isoformat()
    now_iso    = fetched_at

    all_items: list[dict] = []
    seen:      set[str]   = set()

    def add(item: dict) -> None:
        if item["id"] not in seen:
            seen.add(item["id"])
            all_items.append(item)

    # Load existing items, apply retention policy
    if out_path.exists():
        try:
            existing = json.loads(out_path.read_text(encoding="utf-8"))
            for item in existing.get("items", []):
                if not is_expired(item, now_iso) and item["id"] not in seen:
                    seen.add(item["id"])
                    all_items.append(item)
        except Exception as e:
            print(f"[load] Failed to load existing news.json: {e}", file=sys.stderr)

    print("Fetching HackerNews…")
    for item in fetch_hn():
        if item["id"] not in seen:
            item["fetched_at"] = fetched_at
            add(item)

    for source in SOURCES:
        print(f"Fetching {source['name']}…")
        for item in fetch_rss(source):
            if item["id"] not in seen:
                item["fetched_at"] = fetched_at
                add(item)

    all_items.sort(key=lambda x: x.get("published_at", ""), reverse=True)

    # Count by domain for diagnostics
    domain_counts: dict[str, int] = {}
    for it in all_items:
        if it.get("status") == "raw":
            d = it.get("domain", "unknown")
            domain_counts[d] = domain_counts.get(d, 0) + 1

    output = {
        "generated_at": now_iso,
        "period":        datetime.now(timezone.utc).strftime("%Y-%m-%d"),
        "count":         len(all_items),
        "synthesis":     None,
        "items":         all_items,
    }

    # Preserve existing synthesis field if present
    if out_path.exists():
        try:
            old = json.loads(out_path.read_text(encoding="utf-8"))
            if old.get("synthesis"):
                output["synthesis"] = old["synthesis"]
        except Exception:
            pass

    out_path.parent.mkdir(parents=True, exist_ok=True)
    out_path.write_text(json.dumps(output, ensure_ascii=False, indent=2), encoding="utf-8")

    raw_total = sum(domain_counts.values())
    print(f"Done: {len(all_items)} total items, {raw_total} new raw articles today")
    for domain, count in sorted(domain_counts.items()):
        print(f"  {domain}: {count} raw")
    print("Next step: run score_articles.py to run domain competition.")


if __name__ == "__main__":
    main()
