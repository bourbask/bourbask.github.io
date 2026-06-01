# Setup développement local

> Dernière mise à jour : 2026-06-01

---

## Prérequis

| Outil | Version | Installation |
|-------|---------|-------------|
| Rust | stable | `curl https://sh.rustup.rs -sSf \| sh` |
| wasm32 target | — | `rustup target add wasm32-unknown-unknown` |
| Trunk | 0.21.14 | `cargo install --locked trunk --version 0.21.14` |
| Python | 3.12 | via `pyenv` ou package manager |
| Node.js | LTS | pour les outils de qualité (pa11y, Lighthouse CI) |

---

## Installation

```bash
# Cloner le repo
git clone https://github.com/bourbask/bourbask.github.io.git
cd bourbask.github.io

# Dépendances Python (scripts veille)
pip install -r scripts/requirements.txt

# Optionnel — outils de qualité
npm install -g @lhci/cli pa11y
```

---

## Dev server (hot reload)

```bash
trunk serve
# → http://127.0.0.1:9999
# Hot reload sur changements dans src/ et public/
```

Trunk recompile le WASM et recharge le navigateur à chaque modification de fichier Rust. Modifications CSS/assets dans `public/` rechargées sans recompilation.

---

## Build de production

```bash
trunk build --release

# Sortie dans dist/
# dist/index.html
# dist/*.wasm
# dist/*.js
# dist/[public/* copié]
```

---

## Scripts veille technologique

### Tester la synthèse (non-destructif)

```bash
export ANTHROPIC_API_KEY=sk-ant-...
python scripts/test_synthesis.py --days 7
# Affiche le JSON généré sur stdout
# Ne modifie PAS public/news.json
```

### Forcer un fetch manuel

```bash
export ANTHROPIC_API_KEY=sk-ant-...
python scripts/fetch_news.py
# Modifie public/news.json
```

### Forcer la synthèse hebdomadaire

```bash
export ANTHROPIC_API_KEY=sk-ant-...
python scripts/synthesize_news.py
# Modifie public/news.json
```

---

## Structure des branches

```
main        Production — tout push déclenche un déploiement
develop     Intégration — CI Clippy + audit sécurité
feature/*   Fonctionnalités en cours
```

**Workflow type :**

```bash
git checkout -b feature/ma-fonctionnalite develop
# ... développement ...
git push origin feature/ma-fonctionnalite
# PR feature/* → develop (rebase)
# PR develop → main (squash)
```

---

## Lancer les validations localement

### Clippy (linter Rust)

```bash
cargo clippy --target wasm32-unknown-unknown -- -D warnings
```

### Audit sécurité

```bash
cargo install cargo-audit
cargo audit
```

### pa11y (accessibilité) — nécessite le site déployé ou un serveur local

```bash
# Avec trunk serve actif sur :9999
pa11y http://127.0.0.1:9999 --standard WCAG2AA
pa11y http://127.0.0.1:9999/blog --standard WCAG2AA
pa11y http://127.0.0.1:9999/veille --standard WCAG2AA
```

---

## Variables d'environnement

| Variable | Requis pour | Obtenir |
|----------|------------|---------|
| `ANTHROPIC_API_KEY` | Scripts veille (synthèse Claude) | console.anthropic.com |

En CI : secret GitHub `ANTHROPIC_API_KEY`.  
En local : `.env` (non versionné) ou export manuel.

---

## Déclencher les workflows manuellement

Via l'interface GitHub (Actions → workflow → Run workflow) :

| Workflow | Utilité |
|----------|---------|
| `fetch-news.yml` | Forcer un fetch immédiat |
| `synthesize-news.yml` | Forcer la synthèse hebdomadaire |
| `deploy.yml` | Redéployer sans push |
| `quality.yml` | Relancer la validation post-déploiement |

---

## Troubleshooting courant

### `trunk serve` échoue avec une erreur wasm-bindgen

```bash
# Nettoyer le cache de build
rm -rf dist/ target/
trunk serve
```

### `cargo clippy` donne des erreurs sur des crates WASM

S'assurer de cibler le bon target :

```bash
cargo clippy --target wasm32-unknown-unknown
```

### `fetch_news.py` ne génère pas de synthèse

Vérifier que `ANTHROPIC_API_KEY` est défini :

```bash
echo $ANTHROPIC_API_KEY
# Si vide, la synthèse est silencieusement skippée (pas une erreur fatale)
```

### news.json semble vide ou ancien

```bash
# Vérifier la date de génération
python -c "import json; d=json.load(open('public/news.json')); print(d['generated_at'], d['count'], 'items')"
```
