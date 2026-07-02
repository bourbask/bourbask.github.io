.PHONY: help setup setup-rust setup-python serve build fetch score synth synth-ai test clean \
        a11y a11y-dark migrate reprocess discover discover-dry discover-cat article article-dry

TRUNK_VERSION := 0.21.14
VENV          := .venv
PYTHON        := $(VENV)/bin/python3

# ── Default ────────────────────────────────────────────────────────────────────
help:
	@echo ""
	@echo "━━ Setup ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
	@echo "  make setup              Install all deps (Rust + WASM + Trunk + Python venv)"
	@echo "  make setup-rust         Rust toolchain + wasm32 target + Trunk"
	@echo "  make setup-python       Python venv + feedparser, requests, anthropic"
	@echo ""
	@echo "━━ Dev ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
	@echo "  make serve              Dev server http://localhost:9999 (hot reload)"
	@echo "  make build              Production build → dist/"
	@echo "  make clean              Remove dist/ + Rust target/"
	@echo ""
	@echo "━━ Qualité / A11y ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
	@echo "  make a11y               pa11y WCAG2AA — light mode (serveur doit tourner)"
	@echo "  make a11y-dark          pa11y WCAG2AA — dark mode"
	@echo ""
	@echo "━━ Pipeline Veille ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
	@echo "  make fetch              Fetch RSS + HN (sans IA)"
	@echo "  make score              Compétition articles du jour (Haiku)"
	@echo "  make score-dry          Score sans écrire"
	@echo "  make synth              Synthèse générale hebdo (Sonnet, IA exclue)"
	@echo "  make synth-ai           Synthèse IA dédiée (Sonnet, EN, courte)"
	@echo "  make test               Tests pipeline (offline, zéro token IA)"
	@echo "  make migrate            Migration one-shot synthèses (nettoyage historique)"
	@echo "  make reprocess          Re-score + re-synthèse W19/W20/W21"
	@echo ""
	@echo "━━ Articles Blog — Outils Open Source ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
	@echo "  make discover           Découverte hebdo (toutes catégories, Haiku)"
	@echo "  make discover-dry       Aperçu sans écrire"
	@echo "  make discover-cat CAT=ia-local   Découverte pour une catégorie"
	@echo "  make article GITHUB=owner/repo   Génère article + PR (Sonnet)"
	@echo "  make article ID=vaultwarden      Génère depuis tools_candidates.json"
	@echo "  make article-dry GITHUB=owner/repo  Preview sans commit"
	@echo ""
	@echo "  Catégories : devops | securite | productivite | self-hosting | monitoring"
	@echo "               data | iot | impression-3d | admin-quotidien | jeux-video | ia-local"
	@echo ""

# ── Setup ──────────────────────────────────────────────────────────────────────
setup: setup-rust setup-python
	@echo ""
	@echo "✓ Setup complete. Run 'make serve' to start."

setup-rust:
	@echo "→ Rust stable toolchain"
	@rustup default stable
	@echo "→ wasm32-unknown-unknown target"
	@rustup target add wasm32-unknown-unknown
	@echo "→ Trunk $(TRUNK_VERSION)"
	@if ! trunk --version 2>/dev/null | grep -q "$(TRUNK_VERSION)"; then \
		cargo install --locked trunk --version $(TRUNK_VERSION); \
	else \
		echo "  Trunk $(TRUNK_VERSION) already installed, skipping."; \
	fi

setup-python:
	@echo "→ Python venv + dependencies"
	@python3 -m venv $(VENV)
	@$(PYTHON) -m pip install --upgrade pip -q
	@$(PYTHON) -m pip install -r scripts/requirements.txt

# ── Dev / Build ────────────────────────────────────────────────────────────────
serve:
	trunk serve

build:
	trunk build --release

# ── News pipeline ──────────────────────────────────────────────────────────────
fetch:
	$(PYTHON) scripts/fetch_news.py

score:
	@if [ -z "$$ANTHROPIC_API_KEY" ]; then \
		echo "Error: ANTHROPIC_API_KEY is not set"; exit 1; \
	fi
	$(PYTHON) scripts/score_articles.py

a11y:
	@echo "Light mode"
	@for route in / /blog /veille; do pa11y --standard WCAG2AA http://127.0.0.1:9999$$route; done

a11y-dark:
	@echo "Dark mode"
	@for route in / /blog /veille; do pa11y --standard WCAG2AA --config '{"beforeScript":"scripts/pa11y-dark-mode.js"}' http://127.0.0.1:9999$$route; done

score-dry:
	@if [ -z "$$ANTHROPIC_API_KEY" ]; then \
		echo "Error: ANTHROPIC_API_KEY is not set"; exit 1; \
	fi
	$(PYTHON) scripts/score_articles.py --dry-run

synth:
	@if [ -z "$$ANTHROPIC_API_KEY" ]; then \
		echo "Error: ANTHROPIC_API_KEY is not set"; exit 1; \
	fi
	$(PYTHON) scripts/synthesize_news.py --track general

synth-ai:
	@if [ -z "$$ANTHROPIC_API_KEY" ]; then \
		echo "Error: ANTHROPIC_API_KEY is not set"; exit 1; \
	fi
	$(PYTHON) scripts/synthesize_news.py --track ai

ledger:
	$(PYTHON) scripts/extract_ledger.py

ledger-dry:
	$(PYTHON) scripts/extract_ledger.py --dry-run

forecast:
	@if [ -z "$$ANTHROPIC_API_KEY" ]; then \
		echo "Error: ANTHROPIC_API_KEY is not set"; exit 1; \
	fi
	$(PYTHON) scripts/forecast_news.py

forecast-dry:
	@if [ -z "$$ANTHROPIC_API_KEY" ]; then \
		echo "Error: ANTHROPIC_API_KEY is not set"; exit 1; \
	fi
	$(PYTHON) scripts/forecast_news.py --dry-run

# ── Tests (offline, zéro token IA) ──────────────────────────────────────────────
test:
	$(PYTHON) -m pytest scripts/tests -v

migrate:
	$(PYTHON) scripts/migrate_syntheses.py

# ── Tool discovery & article generation ────────────────────────────────────────
discover:
	$(PYTHON) scripts/discover_tools.py

discover-dry:
	$(PYTHON) scripts/discover_tools.py --dry-run

discover-cat:
	@if [ -z "$$CAT" ]; then echo "Usage: make discover-cat CAT=ia-local"; exit 1; fi
	$(PYTHON) scripts/discover_tools.py --category $$CAT

# Generate article for a specific GitHub repo
# Usage: make article GITHUB=dani-garcia/vaultwarden
article:
	@if [ -z "$$GITHUB" ] && [ -z "$$ID" ]; then \
		echo "Usage: make article GITHUB=owner/repo  OR  make article ID=vaultwarden"; exit 1; \
	fi
	@if [ -z "$$ANTHROPIC_API_KEY" ]; then echo "Error: ANTHROPIC_API_KEY not set"; exit 1; fi
	@if [ -n "$$GITHUB" ]; then \
		$(PYTHON) scripts/generate_article.py --github $$GITHUB; \
	else \
		$(PYTHON) scripts/generate_article.py --id $$ID; \
	fi

article-dry:
	@if [ -z "$$GITHUB" ] && [ -z "$$ID" ]; then \
		echo "Usage: make article-dry GITHUB=owner/repo"; exit 1; \
	fi
	@if [ -n "$$GITHUB" ]; then \
		$(PYTHON) scripts/generate_article.py --github $$GITHUB --dry-run; \
	else \
		$(PYTHON) scripts/generate_article.py --id $$ID --dry-run; \
	fi

# Re-score and re-synthesize all weeks after migration
reprocess:
	@if [ -z "$$ANTHROPIC_API_KEY" ]; then \
		echo "Error: ANTHROPIC_API_KEY is not set"; exit 1; \
	fi
	$(PYTHON) scripts/score_articles.py --week 2026-W19
	$(PYTHON) scripts/score_articles.py --week 2026-W20
	$(PYTHON) scripts/score_articles.py --week 2026-W21
	$(PYTHON) scripts/synthesize_news.py --week 2026-W19
	$(PYTHON) scripts/synthesize_news.py --week 2026-W20
	$(PYTHON) scripts/synthesize_news.py --week 2026-W21

# ── Clean ──────────────────────────────────────────────────────────────────────
clean:
	rm -rf dist/
	cargo clean
