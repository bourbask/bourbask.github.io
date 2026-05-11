# bourbask.github.io

Personal portfolio and tech watch — built with Rust, Leptos, and WebAssembly.

**Live:** [bourbask.github.io](https://bourbask.github.io)

---

## What's in here

- Portfolio (about, skills, projects, contact)
- Blog with article routing
- Tech Watch (`/veille`) — daily news feed from RSS/HN, auto-updated via GitHub Actions
- Bilingual (FR/EN) with persistent language preference
- Dark/light theme with system preference detection, no flash on load
- Printable CV generator

---

## Stack

| Layer | Tech |
|-------|------|
| Language | Rust (stable) |
| Framework | Leptos 0.5 (CSR) |
| Bundler | Trunk |
| Target | wasm32-unknown-unknown |
| Styling | CSS3 with custom properties |
| News fetch | Python 3 + feedparser (GitHub Actions cron) |

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
git checkout leptos-wasm

# Start dev server (hot reload at http://127.0.0.1:9999)
trunk serve

# Production build
trunk build --release
# Output: dist/
```

### Useful commands

```bash
trunk serve            # dev server with hot reload
trunk serve --open     # dev server + open browser
trunk build            # debug build
trunk build --release  # optimized build for deployment

cargo clippy           # linting
cargo fmt              # formatting
cargo check            # fast type check (no binary output)
```

---

## Project structure

```
src/
├── lib.rs                    # WASM entry point
├── app.rs                    # Router and top-level contexts
├── components/
│   ├── mod.rs
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
│   ├── veille.rs             # Tech watch page (fetches /news.json)
│   ├── blog/
│   │   ├── mod.rs
│   │   ├── blog_page.rs
│   │   └── article_page.rs
│   └── ui/
│       ├── cv_download.rs
│       ├── lang_toggle.rs
│       └── theme_toggle.rs
├── services/
│   ├── i18n.rs               # Language switching + localStorage persistence
│   ├── theme.rs              # Dark/light theme
│   ├── storage.rs            # localStorage wrapper
│   ├── blog.rs
│   ├── cv.rs
│   └── animations.rs
└── data/
    ├── cv.rs
    ├── articles/
    └── translations/
        ├── en.rs
        └── fr.rs

scripts/
└── fetch_news.py             # Fetches RSS + HackerNews, outputs public/news.json

.github/workflows/
└── fetch-news.yml            # Runs fetch_news.py daily at 06:00 UTC
```

---

## Tech Watch (Veille)

The `/veille` page shows a daily-updated feed of tech news relevant to the stack (Symfony, React, Rust, PHP, DevOps, AI).

**How it works:**

1. GitHub Actions runs `scripts/fetch_news.py` every day at 06:00 UTC
2. The script fetches from HackerNews API + 13 RSS feeds
3. Items are classified by keyword into: `urgent`, `good_news`, `future_watch`, `stack_alt`, `general`
4. Result is committed to `public/news.json` on main
5. The Leptos frontend fetches `/news.json` on page load

**To run the fetch script locally:**

```bash
pip install feedparser requests
python scripts/fetch_news.py
# Output: public/news.json
```

---

## Deployment

The site deploys to GitHub Pages. A deploy workflow (`.github/workflows/deploy.yml`) handles the build and publish. Minimal setup:

```yaml
name: Deploy

on:
  push:
    branches: [main]

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: wasm32-unknown-unknown

      - uses: jetli/trunk-action@v0.4.0
        with:
          version: "latest"

      - run: trunk build --release

      - uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: ./dist
```

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
- The `leptos-wasm` branch is the active development branch. `main` is what GitHub Pages deploys from.
- Codebase is a learning project. Quality varies; a cleanup pass is planned.

---

## License

MIT — see [LICENSE](LICENSE).
