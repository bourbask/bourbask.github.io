"""repair_synthesis_links.py — deterministic URL repair, no network, no tokens."""
import repair_synthesis_links as rsl


SOURCES = [
    {"url": "https://www.infoq.com/news/2026/06/uber-payment-batching-system/?utm=x",
     "title": "Uber Rebuilds Its Payment Batching System"},
    {"url": "https://www.infoq.com/presentations/architecting-deletion-system/",
     "title": "Architecting a Deletion System at Scale"},
    {"url": "https://letsencrypt.org/2026/06/03/pq-certs",
     "title": "Post-Quantum Certificates"},
    {"url": "https://oxide.computer/blog/iddqd-unsafe",
     "title": "iddqd: taming unsafe Rust"},
]


def test_exact_url_left_untouched():
    assert rsl.best_match("Uber", SOURCES[0]["url"], SOURCES) is None


def test_homepage_link_left():
    assert rsl.best_match("oxide", "https://oxide.computer/", SOURCES) is None


def test_external_host_left():
    assert rsl.best_match("Temporal", "https://temporal.io/docs", SOURCES) is None


def test_unique_host_repaired():
    # only one letsencrypt source → confident swap
    out = rsl.best_match("post-quantum", "https://letsencrypt.org/2026/06/post-quantum-future/", SOURCES)
    assert out == "https://letsencrypt.org/2026/06/03/pq-certs"


def test_multi_host_best_slug_wins():
    out = rsl.best_match("Uber", "https://www.infoq.com/news/2026/06/uber-ledger-processing-batching/", SOURCES)
    assert out == SOURCES[0]["url"]


def test_multi_host_low_similarity_left():
    # invented presentation that matches no real source → leave it (honest 404)
    out = rsl.best_match("Google A/B testing",
                         "https://www.infoq.com/presentations/google-ab-testing-global-fleet/", SOURCES)
    assert out is None


def test_anchor_title_rescues_match():
    # slug is garbage but the anchor text matches a source title
    out = rsl.best_match("Architecting a Deletion System",
                         "https://www.infoq.com/presentations/xyz-broken-slug/", SOURCES)
    assert out == SOURCES[1]["url"]


def test_repair_links_counts_and_preserves_anchor():
    content = "See [the Uber rebuild](https://www.infoq.com/news/2026/06/uber-ledger-processing-batching/)."
    out, n = rsl.repair_links(content, SOURCES)
    assert n == 1
    assert "[the Uber rebuild](https://www.infoq.com/news/2026/06/uber-payment-batching-system/?utm=x)" in out


def test_repair_links_leaves_images():
    content = "![hero](https://www.infoq.com/news/2026/06/uber-ledger-processing-batching/)"
    out, n = rsl.repair_links(content, SOURCES)
    assert n == 0          # image links are not touched
    assert out == content
