# bourbask.github.io

Personal portfolio and tech watch — built with Rust, Leptos, and WebAssembly.

**Live:** [www.bourbasquetkev.in](https://www.bourbasquetkev.in) — also served at [bourbask.github.io](https://bourbask.github.io)

---

## What's in here

- Portfolio (about, skills, projects, contact)
- Blog with article routing
- Tech Watch (`/veille`) — daily news feed from RSS/HN, a weekly editorial synthesis, and a dedicated AI watch with its own brief; auto-updated via GitHub Actions
- Bilingual (FR/EN) with persistent language preference
- Dark/light theme with system preference detection, no flash on load
- Printable CV generator

---

## Documentation

In-depth docs live in [`docs/`](docs/):

| Doc | Covers |
|-----|--------|
| [veille-pipeline.md](docs/veille-pipeline.md) | Tech watch pipeline — sources, scoring, AI track, synthesis, schema |
| [testing.md](docs/testing.md) | Token-free pipeline tests (faked client, network guard) |
| [architecture.md](docs/architecture.md) | Frontend Rust/Leptos/WASM — components, services, routing |
| [deployment.md](docs/deployment.md) | CI/CD, GitHub Pages, custom domain |
| [design-system.md](docs/design-system.md) | Design tokens, typography, accessibility |
| [dev-setup.md](docs/dev-setup.md) | Local setup, commands, troubleshooting |

---

## Stack

| Layer | Tech |
|-------|------|
| Language | Rust (stable) |
| Framework | Leptos 0.8 (CSR) |
| Bundler | Trunk 0.21.14 |
| Target | wasm32-unknown-unknown |
| Styling | CSS3 with custom properties |
| News fetch + scoring | Python 3 + feedparser + Claude Haiku (daily, GitHub Actions) |
| News synthesis | Python 3 + Claude Sonnet (general: Monday; AI brief: Mon + Thu) |
| Pipeline tests | pytest — offline, no API key, zero AI tokens |

---

## Setup

### Standard install (most systems)

```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Trunk
cargo install --locked trunk
```

### Arch / Manjaro

The `rustup` package from pacman installs shims in `/usr/bin` (cargo, rustc) that point to rustup itself. Running `cargo` without a configured toolchain causes infinite recursion.

```bash
# Check if this is your situation
file /usr/bin/cargo   # → "symbolic link to rustup" = you have this issue

# Fix: install the stable toolchain and set it as default
rustup toolchain install stable
rustup target add wasm32-unknown-unknown
rustup default stable

# Verify
cargo --version   # should print "cargo 1.x.x ..."

# Install Trunk — via AUR (faster, no compile)
yay -S trunk
# or
paru -S trunk

# Alternatively, compile from source (takes ~5-10 min)
cargo install --locked trunk
```

> Do not use `rustup toolchain link system /usr`. It links the shim as the toolchain, causing the same recursion.

---

## Development

```bash
# Clone
git clone https://github.com/bourbask/bourbask.github.io.git
cd bourbask.github.io

# Start dev server (hot reload at http://127.0.0.1:9999)
trunk serve

# Production build
trunk build --release
# Output: dist/
```

### Useful commands

```bash
trunk serve            # dev server with hot reload (http://127.0.0.1:9999)
trunk build            # debug build
trunk build --release  # optimized build for deployment

cargo clippy --release --target wasm32-unknown-unknown  # linting (matches CI)
cargo fmt              # formatting
cargo check            # fast type check (no binary output)
```

---

## Quality checks

The CI pipeline runs accessibility, performance, HTML validity, and security header checks automatically after each deploy to `main`. Run the same checks locally before pushing to catch issues early.

### Install tools (once)

```bash
npm install -g @lhci/cli pa11y
```

Docker is optional — needed only for the W3C HTML validator against localhost (the CI run uses the W3C public API instead).

### Run before pushing

```bash
./scripts/quality_check.sh
```

The script starts `trunk serve` if not already running, runs all checks against `http://localhost:9999`, then exits and cleans up.

```bash
# Partial runs
./scripts/quality_check.sh --skip-w3c          # no Docker required
./scripts/quality_check.sh --skip-lhci         # skip Lighthouse
./scripts/quality_check.sh --skip-a11y         # skip pa11y
```

### What each check covers

| Check | Tool | Fails on |
|-------|------|----------|
| Performance, SEO, best practices | Lighthouse CI (`@lhci/cli`) | perf < 0.80, SEO < 0.90 |
| Accessibility WCAG 2.1 AA | pa11y + axe-core | any WCAG2AA error |
| HTML validity | W3C Nu Validator | any HTML error |
| HTTP security headers | curl | missing `X-Frame-Options`, `X-Content-Type-Options`, `Strict-Transport-Security` |

Thresholds are defined in `.lighthouserc.json`.

### CI equivalent

The same checks run in `.github/workflows/quality.yml`, triggered automatically after each successful deploy to `main`. Results are available in the Actions tab; Lighthouse reports are uploaded as artifacts (30-day retention).

---

## Project structure

```
src/
├── lib.rs                    # WASM entry point
├── app.rs                    # Router and top-level contexts
├── components/
│   ├── hero.rs
│   ├── about.rs
│   ├── skills.rs
│   ├── projects.rs
│   ├── interests.rs
│   ├── contact.rs
│   ├── navigation.rs
│   ├── mobile_nav.rs
│   ├── footer.rs
│   ├── not_found.rs
│   ├── veille.rs             # Tech watch page + synthesis detail
│   ├── blog/
│   │   ├── blog_page.rs
│   │   └── article_page.rs
│   └── ui/
│       └── cv_download.rs
├── services/
│   ├── i18n.rs               # Language switching + localStorage persistence
│   ├── theme.rs              # Dark/light theme
│   ├── storage.rs            # localStorage wrapper
│   ├── blog.rs
│   └── cv.rs
└── data/
    ├── cv.rs
    ├── articles/
    └── translations/
        ├── en.rs
        └── fr.rs

scripts/
├── fetch_news.py             # RSS + HackerNews → public/news.json (no AI)
├── score_articles.py         # Per-domain scoring + selection (Claude Haiku)
├── synthesize_news.py        # Weekly general synthesis + dedicated AI brief (Claude Sonnet)
├── repair_synthesis_links.py # One-shot repair of source links in syntheses (no AI)
├── check_feeds.py            # Feed reachability smoke check (no AI)
├── discover_tools.py         # Open-source tool discovery (tools pipeline)
├── generate_article.py       # Blog article generation for discovered tools
├── tests/                    # pytest suite — offline, zero AI tokens
└── quality_check.sh          # Local quality checks (mirrors CI)

public/
├── news.json                 # Generated — do not edit manually
├── css/                      # Design system (variables, components, pages)
└── fonts/                    # Self-hosted Literata variable font (OFL-1.1)

.github/workflows/
├── deploy.yml                # Build + deploy to GH Pages on push to main
├── ci.yml                    # Clippy + cargo audit on develop / PR to main
├── news-pipeline.yml         # Daily fetch + score; general synthesis (Mon), AI brief (Mon+Thu)
├── tests.yml                 # pytest pipeline tests (no API key → zero tokens)
├── feeds-smoke.yml           # Weekly feed reachability check (no AI)
├── tools-pipeline.yml        # Weekly open-source tool discovery + article PR
└── quality.yml               # Post-deploy quality checks (Lighthouse, pa11y, W3C, headers)
```

---

## Tech Watch (Veille)

The `/veille` page shows a daily news feed, a weekly editorial synthesis, and a dedicated **AI watch** (own tab + own brief). Everything is generated by `news-pipeline.yml`.

**Daily — fetch + score (no synthesis):**

1. `fetch_news.py` pulls HackerNews + ~45 RSS feeds across 6 domains (`dev_stack`, `ai`, `security`, `health_science`, `business_market`, `architecture`), classifies, and commits to `public/news.json`.
2. `score_articles.py` scores each domain with Claude Haiku and selects a general top 5 (AI excluded) plus a dedicated AI top.

**Synthesis:**

- **General** (Monday) — `synthesize_news.py --track general` writes a bilingual (FR + EN) editorial synthesis. AI-domain articles are excluded.
- **AI brief** (Monday + Thursday) — `synthesize_news.py --track ai` writes a short, token-lean, English-only brief (state of the art → implications → developer actions).
- Source links use `srcref:<id>` tokens swapped for exact feed URLs after generation (no hallucinated 404s). Hero illustrations pass a relevance gate.

Synthesis cards appear in the feed; clicking opens a full Markdown detail page. The "IA" tab isolates the AI watch.

**Run locally (`make`):**

```bash
make fetch        # RSS + HN, no AI
make score        # daily scoring (needs ANTHROPIC_API_KEY)
make synth        # general weekly synthesis
make synth-ai     # dedicated AI brief
make test         # pipeline tests — offline, no API key, zero tokens
```

Repair broken source links in already-published syntheses (deterministic, no AI):

```bash
python scripts/repair_synthesis_links.py --dry-run
```

See [`docs/veille-pipeline.md`](docs/veille-pipeline.md) for the full pipeline and [`docs/testing.md`](docs/testing.md) for the token-free test design.

---

## Deployment

Push to `main` triggers `.github/workflows/deploy.yml`, which builds with Trunk and deploys to GitHub Pages.

**Custom domain:** served at `www.bourbasquetkev.in`. A `CNAME` file (copied to the build root via `data-trunk`) keeps the domain pinned across Actions deploys. DNS and cohabitation with the homelab VPS are documented in [`docs/deployment.md`](docs/deployment.md).

The CI workflow (`.github/workflows/ci.yml`) runs on every push to `develop` and every PR to `main`:
- `cargo clippy --release --target wasm32-unknown-unknown -D warnings`
- `cargo audit` (RUSTSEC advisory check)

**Important:** Do not set `data-wasm-opt` in `index.html`. `wasm-opt` without `--enable-reference-types` corrupts the wasm-bindgen externref table at runtime (blank page).

---

## i18n

Translations live in `src/data/translations/en.rs` and `fr.rs` as `HashMap<&'static str, &'static str>`. The selected language is persisted in `localStorage` (`portfolio_language` key) and restored on page load.

To add a key:

```rust
// en.rs
map.insert("my.key", "My value");

// fr.rs
map.insert("my.key", "Ma valeur");
```

Usage in a component:

```rust
let i18n = use_context::<I18nService>().unwrap();
view! { <p>{move || i18n.t("my.key")}</p> }
```

---

## Notes

- No SSR, no server functions — pure CSR WASM. Dynamic data (news feed) is pre-fetched by GitHub Actions and served as static JSON.
- GitHub Pages source must be set to "GitHub Actions" in repository settings (not "Deploy from branch").

---

## License

MIT — see [LICENSE](LICENSE).
