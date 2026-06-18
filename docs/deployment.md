# CI/CD pipeline — deployment & quality

> Last updated: 2026-06-18

---

## Overview

```
Push to main
      │
      ├──→ deploy.yml        Trunk build → GitHub Pages
      │         │
      │         └──→ quality.yml   Lighthouse + pa11y + W3C + headers
      │
PR to main
      │
      └──→ ci.yml            Clippy + rustsec

Daily cron (06:00 UTC)
      └──→ news-pipeline.yml   fetch + score; general synthesis (Mon), AI brief (Mon+Thu)

Weekly cron
      ├──→ tools-pipeline.yml  Open-source tool discovery + article PR
      └──→ feeds-smoke.yml     Feed reachability check (no AI)

Push / PR touching scripts/**
      └──→ tests.yml           pytest pipeline (no API key → zero tokens)
```

See [veille-pipeline.md](veille-pipeline.md) for the tech-watch workflow details.

---

## Workflows

### `ci.yml` — pre-merge validation

**Trigger:** push to `develop`, PR to `main`

```yaml
jobs:
  clippy:
    - rustup + wasm32-unknown-unknown target
    - cargo clippy --target wasm32-unknown-unknown -- -D warnings
    # -D warnings: any warning fails CI

  audit:
    - cargo install cargo-audit
    - cargo audit
    # Checks known vulnerabilities (RustSec advisory DB)
```

**Rule:** no PR merges to `main` if `ci.yml` fails.

---

### `deploy.yml` — build & deploy

**Trigger:** push to `main`, workflow_dispatch

```yaml
permissions:
  contents: read
  pages: write
  id-token: write

jobs:
  build:
    - actions/checkout
    - rustup (stable + wasm32-unknown-unknown)
    - Swatinem/rust-cache            # Cache Cargo registry + target/
    - install trunk v0.21.14         # Pinned version for reproducibility
    - trunk build --release          # Compile src/ + copy public/ → dist/
    - upload-pages-artifact          # Upload dist/ to the GitHub Pages artifact store

  deploy:
    needs: build
    environment: github-pages
    - actions/deploy-pages           # Publish the artifact to GitHub Pages
```

**Typical duration:** 4–8 min (~3–5 min Rust compilation, reduced by the cache).

**Production URL:** https://www.bourbasquetkev.in (custom domain, see below) — also served at https://bourbask.github.io

---

### `quality.yml` — post-deploy validation

**Trigger:** workflow_call (called by deploy.yml after success), workflow_dispatch

```yaml
jobs:
  validate:
    steps:
      1. Wait for CDN propagation (up to 2 min, HTTP polling)

      2. Lighthouse CI
         - npm install -g @lhci/cli
         - lhci autorun (config in .lighthouserc.json)
         - Thresholds: performance ≥ 0.85, accessibility ≥ 0.95,
                       best-practices ≥ 0.9, seo ≥ 0.9

      3. pa11y — WCAG 2.1 AA
         - npm install -g pa11y
         - 3 routes audited: /, /blog, /veille
         - WCAG2AA standard, level AA required

      4. W3C Nu HTML Validator
         - curl validator.w3.org/nu/?doc=<url>
         - Zero errors required

      5. HTTP security headers
         - Checks for: Content-Security-Policy, X-Frame-Options,
                       X-Content-Type-Options, Referrer-Policy
```

**Lighthouse config:** `.lighthouserc.json` at the project root.

---

## Lighthouse config (`.lighthouserc.json`)

```json
{
  "ci": {
    "collect": { "url": ["https://bourbask.github.io/"], "numberOfRuns": 3 },
    "assert": {
      "assertions": {
        "categories:performance": ["error", { "minScore": 0.85 }],
        "categories:accessibility": ["error", { "minScore": 0.95 }],
        "categories:best-practices": ["error", { "minScore": 0.90 }],
        "categories:seo": ["error", { "minScore": 0.90 }]
      }
    }
  }
}
```

---

## Required secrets and permissions

| Secret / Permission | Workflows | Role |
|---------------------|-----------|------|
| `ANTHROPIC_API_KEY` | news-pipeline, tools-pipeline | Claude API calls (Haiku/Sonnet) |
| `contents: write` | news-pipeline | Auto-commit news.json |
| `pages: write` | deploy | GitHub Pages publishing |
| `id-token: write` | deploy | OIDC for GitHub Pages |

The `tests.yml` workflow gets **no** `ANTHROPIC_API_KEY` — proof it cannot spend tokens.

---

## Rust cache (Swatinem/rust-cache)

Covers `~/.cargo/registry`, `~/.cargo/git`, and `target/`.
Lifetime: 7 days (GitHub default), invalidated when `Cargo.lock` changes.
Typical gain: compilation from 15–20 min down to 4–6 min after the first build.

---

## Trunk (pinned: 0.21.14)

Pinned in `deploy.yml` to avoid silent regressions on Trunk updates. Bump manually and test locally before changing the CI version.

---

## Custom domain (`www.bourbasquetkev.in`)

The site is served at `www.bourbasquetkev.in` in addition to `bourbask.github.io`.

### Repo side
- `CNAME` file (repo root) containing `www.bourbasquetkev.in`.
- Copied to the **build root** via `index.html`: `<link data-trunk rel="copy-file" href="CNAME" />`.
  Required: without this file in the artifact, GitHub unsets the custom domain on every Actions deploy.

### GitHub side
Settings → Pages → Custom domain = `www.bourbasquetkev.in` → verification (TXT) → **Enforce HTTPS** (auto Let's Encrypt cert).

### DNS side (OVH) — cohabiting with the homelab VPS
The domain also hosts VPS services (plex, bitwarden, traefik, nextcloud…) via subdomains plus a `*` wildcard → VPS. The only site-related records:

```
www   CNAME   bourbask.github.io.
@     A       185.199.108.153   (+ .109 / .110 / .111 .153)   → apex, redirects to www
```

- An explicit `www` wins over the wildcard → the other subdomains stay on the VPS, untouched.
- The apex on GitHub IPs replaces any previous host (e.g. leftover Vercel `76.76.21.21` must be removed).
- **Do not touch:** `MX`, `SPF`, `DKIM`, the `*` wildcard, or the VPS subdomains.
- Watch out for `CNAME` records pointing at the apex (e.g. `ftp`): repoint them to an `A` record on the VPS, otherwise they follow the apex to GitHub.

---

## Branching & delivery flow

```
feature/xxx  →  develop  →  main
                  ↑            ↑
               ci.yml      deploy.yml
               (clippy)    (build + deploy)
                            quality.yml
                            (validation)
```

- **`develop`**: integration branch — Clippy CI + audit
- **`main`**: production branch — every push triggers a deploy

Merge rules: rebase for feature→develop, squash for develop→main.

---

## Reproducibility

- **`Cargo.lock`** committed → identical artifact on every build
- **Pinned Trunk version** → stable bundling behavior
- **`rust-cache`** keyed on `Cargo.lock` hash → auto-invalidation on dependency changes

---

## Rollback

GitHub Pages keeps the artifact of the last successful deploy. On a regression:

1. Identify the culprit commit via `git log`
2. `git revert <commit>` + push to `main`
3. `deploy.yml` re-runs automatically

Or force manually via **GitHub → Actions → deploy.yml → Run workflow** on the previous commit.
