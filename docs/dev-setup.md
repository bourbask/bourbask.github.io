# Local development setup

> Last updated: 2026-06-18

---

## Prerequisites

| Tool | Version | Install |
|------|---------|---------|
| Rust | stable | `curl https://sh.rustup.rs -sSf \| sh` |
| wasm32 target | — | `rustup target add wasm32-unknown-unknown` |
| Trunk | 0.21.14 | `cargo install --locked trunk --version 0.21.14` |
| Python | 3.12 | via `pyenv` or your package manager |
| Node.js | LTS | for the quality tools (pa11y, Lighthouse CI) |

---

## Install

```bash
# Clone the repo
git clone https://github.com/bourbask/bourbask.github.io.git
cd bourbask.github.io

# Python dependencies (tech watch scripts)
pip install -r scripts/requirements.txt

# Optional — quality tools
npm install -g @lhci/cli pa11y
```

---

## Dev server (hot reload)

```bash
trunk serve
# → http://127.0.0.1:9999
# Hot reload on changes in src/ and public/
```

Trunk recompiles the WASM and reloads the browser on every Rust file change. CSS/asset changes in `public/` reload without recompilation.

---

## Production build

```bash
trunk build --release

# Output in dist/
# dist/index.html
# dist/*.wasm
# dist/*.js
# dist/[public/* copied]
```

---

## Tech watch scripts

`make` targets (details in [veille-pipeline.md](veille-pipeline.md)):

```bash
make fetch        # RSS + HN → public/news.json (no AI)
make score        # daily scoring (Claude Haiku)            — ANTHROPIC_API_KEY required
make score-dry    # scoring without writing
make synth        # general weekly synthesis (--track general)
make synth-ai     # dedicated AI brief (--track ai)
make test         # pipeline tests — offline, no API key, zero tokens
```

### Repair source links in older syntheses (deterministic, no AI)

```bash
python scripts/repair_synthesis_links.py --dry-run   # preview
python scripts/repair_synthesis_links.py             # apply
```

### Check feed reachability (no AI)

```bash
python scripts/check_feeds.py
```

---

## Branch structure

```
main        Production — every push triggers a deploy
develop     Integration — Clippy CI + security audit
feature/*   Work in progress
```

**Typical workflow:**

```bash
git checkout -b feature/my-feature develop
# ... development ...
git push origin feature/my-feature
# PR feature/* → develop (rebase)
# PR develop → main (squash)
```

---

## Run the checks locally

### Clippy (Rust linter)

```bash
cargo clippy --target wasm32-unknown-unknown -- -D warnings
```

### Security audit

```bash
cargo install cargo-audit
cargo audit
```

### pa11y (accessibility) — needs the deployed site or a local server

```bash
# With trunk serve running on :9999
pa11y http://127.0.0.1:9999 --standard WCAG2AA
pa11y http://127.0.0.1:9999/blog --standard WCAG2AA
pa11y http://127.0.0.1:9999/veille --standard WCAG2AA
```

### Pipeline tests (no API key, zero tokens)

```bash
make test
# or
python -m pytest scripts/tests -v
```

---

## Environment variables

| Variable | Required for | Get it |
|----------|--------------|--------|
| `ANTHROPIC_API_KEY` | Tech watch scripts (scoring + synthesis) | console.anthropic.com |

In CI: GitHub secret `ANTHROPIC_API_KEY`.
Locally: `.env` (untracked) or manual export.
The pipeline tests need **no** key.

---

## Trigger workflows manually

Via the GitHub UI (Actions → workflow → Run workflow):

| Workflow | Purpose |
|----------|---------|
| `news-pipeline.yml` | Force a fetch/score; inputs for `force_synthesis`, `force_ai_synthesis`, `score_date`, `score_week` |
| `tools-pipeline.yml` | Force open-source tool discovery |
| `feeds-smoke.yml` | Check feed reachability |
| `deploy.yml` | Redeploy without a push |
| `quality.yml` | Re-run post-deploy validation |

---

## Common troubleshooting

### Arch / Manjaro — `cargo` recursion

The pacman `rustup` package installs `/usr/bin` shims that point back to rustup. Running `cargo` with no configured toolchain recurses infinitely.

```bash
file /usr/bin/cargo   # "symbolic link to rustup" = affected
rustup toolchain install stable
rustup target add wasm32-unknown-unknown
rustup default stable     # or: rustup override set stable (repo-local)
cargo --version           # should print a real version
```

Do not use `rustup toolchain link system /usr` — it links the shim as the toolchain and causes the same recursion.

### Blank page after build

Do not set `data-wasm-opt` in `index.html`. `wasm-opt` without `--enable-reference-types` corrupts the wasm-bindgen externref table at runtime.
