# Pipeline CI/CD — Déploiement & Qualité

> Dernière mise à jour : 2026-06-01

---

## Vue d'ensemble

```
Push sur main
      │
      ├──→ deploy.yml        Build Trunk → GitHub Pages
      │         │
      │         └──→ quality.yml   Lighthouse + pa11y + W3C + headers
      │
PR vers main
      │
      └──→ ci.yml            Clippy + rustsec

Cron quotidien (06:00 UTC)
      └──→ fetch-news.yml    RSS + HN + synthèse quotidienne

Cron hebdo (lundi 07:00 UTC)
      └──→ synthesize-news.yml   Synthèse éditoriale Claude
```

Voir [veille-pipeline.md](veille-pipeline.md) pour le détail des workflows de veille.

---

## Workflows

### `ci.yml` — Validation pré-merge

**Déclencheur :** push sur `develop`, PR vers `main`

```yaml
jobs:
  clippy:
    - rustup + wasm32-unknown-unknown target
    - cargo clippy --target wasm32-unknown-unknown -- -D warnings
    # -D warnings : tout warning = erreur de CI

  audit:
    - cargo install cargo-audit
    - cargo audit
    # Vérifie les vulnérabilités connues (RustSec advisory DB)
```

**Règle :** aucun PR ne merge sur `main` si `ci.yml` échoue.

---

### `deploy.yml` — Build & déploiement

**Déclencheur :** push sur `main`, workflow_dispatch

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
    - install trunk v0.21.14         # Version fixée pour reproductibilité
    - trunk build --release          # Compile src/ + copie public/ → dist/
    - upload-pages-artifact          # Upload dist/ vers GitHub Pages artifact store

  deploy:
    needs: build
    environment: github-pages
    - actions/deploy-pages           # Publie l'artifact sur GitHub Pages
```

**Durée typique :** 4–8 min (dont ~3–5 min compilation Rust, réduite par le cache).

**URL de production :** https://bourbask.github.io

---

### `quality.yml` — Validation post-déploiement

**Déclencheur :** workflow_call (appelé par deploy.yml après succès), workflow_dispatch

```yaml
jobs:
  validate:
    steps:
      1. Attendre propagation CDN (jusqu'à 2 min, polling HTTP)

      2. Lighthouse CI
         - npm install -g @lhci/cli
         - lhci autorun (config dans .lighthouseci/)
         - Seuils : performance ≥ 0.85, accessibility ≥ 0.95,
                    best-practices ≥ 0.9, seo ≥ 0.9
         - Résultats stockés dans .lighthouseci/

      3. pa11y — WCAG 2.1 AA
         - npm install -g pa11y
         - 3 routes auditées :
             pa11y https://bourbask.github.io/
             pa11y https://bourbask.github.io/blog
             pa11y https://bourbask.github.io/veille
         - Standard WCAG2AA, niveau AA requis

      4. W3C Nu HTML Validator
         - curl validator.w3.org/nu/?doc=https://bourbask.github.io/
         - Zéro erreur requise

      5. En-têtes HTTP de sécurité
         - Vérifie la présence de :
             Content-Security-Policy
             X-Frame-Options
             X-Content-Type-Options
             Referrer-Policy
```

**Configuration Lighthouse :** `.lighthouserc.json` à la racine du projet.

---

## Configuration Lighthouse (`.lighthouserc.json`)

```json
{
  "ci": {
    "collect": {
      "url": ["https://bourbask.github.io/"],
      "numberOfRuns": 3
    },
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

## Secrets et permissions requis

| Secret / Permission | Workflows | Rôle |
|---------------------|-----------|------|
| `ANTHROPIC_API_KEY` | fetch-news, synthesize-news | Appels API Claude |
| `contents: write` | fetch-news, synthesize-news | Auto-commit news.json |
| `pages: write` | deploy | Publication GitHub Pages |
| `id-token: write` | deploy | OIDC pour GitHub Pages |

---

## Cache Rust (Swatinem/rust-cache)

Le cache couvre :
- `~/.cargo/registry` — crates téléchargées
- `~/.cargo/git` — dépendances git
- `target/` — artefacts compilés

**Durée de vie :** 7 jours GitHub (par défaut), invalide si `Cargo.lock` change.  
**Gain typique :** réduit la compilation de 15–20 min à 4–6 min après le premier build.

---

## Trunk (version fixée : 0.21.14)

Version fixée dans `deploy.yml` pour éviter les régressions silencieuses lors des mises à jour de Trunk. Mettre à jour manuellement et tester localement avant de changer la version en CI.

```yaml
# Dans deploy.yml — mise à jour manuelle requise
- name: Install Trunk
  run: cargo install --locked trunk --version 0.21.14
```

---

## Branching & flux de livraison

```
feature/xxx  →  develop  →  main
                  ↑            ↑
               ci.yml      deploy.yml
               (clippy)    (build + deploy)
                            quality.yml
                            (validation)
```

- **`develop`** : branche d'intégration — CI Clippy + audit
- **`main`** : branche de production — tout push déclenche un déploiement

Voir [feedback_pr_workflow.md](../memory/feedback_pr_workflow.md) pour les règles de merge (rebase feature→develop, squash develop→main).

---

## Reproductibilité

- **`Cargo.lock`** versionné → même artefact à chaque build
- **Trunk version fixée** → comportement de bundling stable
- **`rust-cache`** par `Cargo.lock` hash → invalidation automatique sur changement de deps

---

## Rollback

GitHub Pages garde l'artifact du dernier déploiement réussi. En cas de régression :

1. Identifier le commit coupable via `git log`
2. `git revert <commit>` + push sur `main`
3. `deploy.yml` repart automatiquement

Ou forcer manuellement via **GitHub → Actions → deploy.yml → Run workflow** sur le commit précédent.
