#!/usr/bin/env python3
"""
Smoke-check every RSS/feed URL in fetch_news.SOURCES (no AI, no tokens).
Reports unreachable / empty feeds — useful after adding new sources (e.g. AI labs).
Exit code 1 only if --strict and at least one feed is broken.
"""
import argparse
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

try:
    import feedparser
    import requests
except ImportError:
    print("Missing deps: pip install feedparser requests", file=sys.stderr)
    sys.exit(1)

from fetch_news import SOURCES

UA = {"User-Agent": "Mozilla/5.0 (compatible; bourbask-veille-smoke/1.0)"}


def check(url: str) -> tuple[bool, str]:
    try:
        resp = requests.get(url, timeout=12, headers=UA, allow_redirects=True)
        if resp.status_code >= 400:
            return False, f"HTTP {resp.status_code}"
        feed = feedparser.parse(resp.content)
        n = len(feed.entries)
        if n == 0:
            return False, "0 entries"
        return True, f"{n} entries"
    except Exception as e:
        return False, f"error: {e}"


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--strict", action="store_true", help="exit 1 if any feed is broken")
    args = ap.parse_args()

    broken = []
    for src in SOURCES:
        ok, detail = check(src["url"])
        mark = "OK  " if ok else "DEAD"
        print(f"[{mark}] [{src['domain']:<16}] {src['name']:<22} {detail}")
        if not ok:
            broken.append(src["name"])

    print(f"\n{len(SOURCES) - len(broken)}/{len(SOURCES)} feeds OK.")
    if broken:
        print("Broken: " + ", ".join(broken))
    if broken and args.strict:
        sys.exit(1)


if __name__ == "__main__":
    main()
