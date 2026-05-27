# bourbask.github.io

Personal portfolio and tech watch — built with Rust, Leptos, and WebAssembly.

**Live:** [bourbask.github.io](https://bourbask.github.io)

---

## What's in here

- Portfolio (about, skills, projects, contact)
- Blog with article routing
- Tech Watch (`/veille`) — daily news feed from RSS/HN + weekly AI synthesis, auto-updated via GitHub Actions
- Bilingual (FR/EN) with persistent language preference
- Dark/light theme with system preference detection, no flash on load
- Printable CV generator

---

## Stack

| Layer | Tech |
|-------|------|
| Language | Rust (stable) |
| Framework | Leptos 0.8 (CSR) |
| Bundler | Trunk 0.21.14 |
| Target | wasm32-unknown-unknown |
| Styling | CSS3 with custom properties |
| News fetch | Python 3 + feedparser (GitHub Actions cron) |
| News synthesis | Python 3 + Claude Haiku API (GitHub Actions, Monday 07:00 UTC) |

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
├── fetch_news.py             # Fetches RSS + HackerNews → public/news.json
├── synthesize_news.py        # Generates weekly AI synthesis via Claude Haiku
├── test_synthesis.py         # Local synthesis test (no file write)
└── quality_check.sh          # Local quality checks (mirrors CI)

public/
├── news.json                 # Generated — do not edit manually
├── css/                      # Design system (variables, components, pages)
└── fonts/                    # Self-hosted Literata variable font (OFL-1.1)

.github/workflows/
├── deploy.yml                # Build + deploy to GH Pages on push to main
├── ci.yml                    # Clippy + cargo audit on develop / PR to main
├── fetch-news.yml            # Daily news fetch (06:00 UTC)
├── synthesize-news.yml       # Weekly synthesis (Monday 07:00 UTC) + workflow_dispatch
└── quality.yml               # Post-deploy quality checks (Lighthouse, pa11y, W3C, headers)
```

---

## Tech Watch (Veille)

The `/veille` page shows a daily-updated feed of tech news and a weekly AI-written synthesis.

**News feed:**

1. GitHub Actions runs `scripts/fetch_news.py` every day at 06:00 UTC
2. Fetches from HackerNews API + 30 RSS feeds (academic, institutional, official lang/framework blogs, multilingual sources)
3. Items are classified into: `urgent`, `good_news`, `future_watch`, `stack_alt`, `general`
4. Result is committed to `public/news.json` on main with a 14-day sliding window

**Weekly synthesis:**

1. `synthesize-news.yml` runs every Monday at 07:00 UTC (or manually via `workflow_dispatch`)
2. `scripts/synthesize_news.py` reads articles from the past 7 days and calls Claude Haiku
3. Generates a bilingual Markdown article (FR + EN) in editorial style
4. Writes a `type: "synthesis"` item to `news.json` and tags source articles with a `synthesis_id`
5. Synthesis cards appear in the feed; clicking "Lire la synthèse" opens a full Markdown detail page

**To run locally:**

```bash
# Fetch news
pip install feedparser requests
python scripts/fetch_news.py

# Test synthesis output (no file write)
ANTHROPIC_API_KEY=your_key python scripts/test_synthesis.py --days 7

# Trigger synthesis in CI manually
gh workflow run synthesize-news.yml --ref main
```

---

## Deployment

Push to `main` triggers `.github/workflows/deploy.yml`, which builds with Trunk and deploys to GitHub Pages.

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
