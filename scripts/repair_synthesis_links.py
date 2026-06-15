#!/usr/bin/env python3
"""
One-shot repair of broken source links in already-generated syntheses.
NO AI, NO tokens — purely deterministic.

Old syntheses (pre-srcref) contain inline links whose URLs the model hallucinated
or mangled (404s). But each synthesis carries its real source URLs in `sources[]`.
For every inline link that points at the SAME HOST as one of those sources but isn't
an exact match, we swap in the correct feed URL (best path-similarity match).
Links with no matching source host (external projects) are left untouched.

Usage:
  python scripts/repair_synthesis_links.py --dry-run
  python scripts/repair_synthesis_links.py
"""
import argparse
import difflib
import json
import os
import re
import sys
from pathlib import Path
from urllib.parse import urlparse

# Markdown links that are NOT images (no leading '!').
_LINK_RE = re.compile(r"(?<!\!)\[([^\]]+)\]\((https?://[^)]+)\)")

# When several sources share a host, a guess below this score is too risky — a
# wrong-topic 200 is worse than an honest 404, so we leave the link as-is.
_MIN_SCORE = 0.45


def _host(url: str) -> str:
    return urlparse(url).netloc.lower().removeprefix("www.")


def _slug(url: str) -> str:
    """Last non-empty path segment — the topical part, free of section prefixes."""
    segs = [s for s in urlparse(url).path.split("/") if s]
    return segs[-1] if segs else ""


def _ratio(a: str, b: str) -> float:
    return difflib.SequenceMatcher(None, a.lower(), b.lower()).ratio()


def best_match(anchor: str, url: str, sources: list[dict]) -> str | None:
    """
    Best correct URL for a (possibly broken) link, or None to leave it untouched.
    Scores each same-host source on max(slug similarity, anchor-vs-title similarity),
    so a section prefix like /presentations/ can't inflate a wrong match.
    """
    urls = [s.get("url", "") for s in sources]
    if url in urls:
        return None  # already correct
    if urlparse(url).path in ("", "/"):
        return None  # bare homepage link — intentional, never rewrite to an article
    same_host = [s for s in sources if _host(s.get("url", "")) == _host(url)]
    if not same_host:
        return None  # external link with no known source → leave alone

    def score(s: dict) -> float:
        return max(
            _ratio(_slug(s.get("url", "")), _slug(url)),
            _ratio(s.get("title", ""), anchor) if anchor else 0.0,
        )

    best = max(same_host, key=score)
    if len(same_host) == 1 or score(best) >= _MIN_SCORE:
        return best.get("url")
    return None


def repair_links(content: str, sources: list[dict]) -> tuple[str, int]:
    """Repair non-image links in `content` against the synthesis source list."""
    if not content:
        return content, 0
    fixed = 0

    def repl(m: "re.Match") -> str:
        nonlocal fixed
        text, url = m.group(1), m.group(2)
        better = best_match(text, url, sources)
        if better and better != url:
            fixed += 1
            return f"[{text}]({better})"
        return m.group(0)

    return _LINK_RE.sub(repl, content), fixed


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--dry-run", action="store_true", help="report changes without writing")
    args = ap.parse_args()

    path = Path(os.environ.get("NEWS_JSON_PATH") or (Path(__file__).parent.parent / "public" / "news.json"))
    data = json.loads(path.read_text(encoding="utf-8"))

    total = 0
    for item in data.get("items", []):
        if item.get("type") != "synthesis":
            continue
        sources = [s for s in item.get("sources", []) if s.get("url")]
        if not sources:
            continue
        n = 0
        for field in ("content_fr", "content_en"):
            new, fixed = repair_links(item.get(field, ""), sources)
            item[field] = new
            n += fixed
        total += n
        print(f"[{item['id']}] {n} link(s) repaired ({len(sources)} source URLs)")

    if args.dry_run:
        print(f"\n[dry-run] {total} link(s) would be repaired. Not writing.")
        return

    path.write_text(json.dumps(data, ensure_ascii=False, indent=2), encoding="utf-8")
    print(f"\nDone: {total} link(s) repaired across syntheses.")


if __name__ == "__main__":
    main()
