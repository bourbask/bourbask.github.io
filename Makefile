.PHONY: help setup setup-rust setup-python serve build fetch score synth clean

TRUNK_VERSION := 0.21.14
VENV          := .venv
PYTHON        := $(VENV)/bin/python3

# ── Default ────────────────────────────────────────────────────────────────────
help:
	@echo ""
	@echo "  make setup        Install all dependencies (Rust + WASM target + Trunk + Python)"
	@echo "  make setup-rust   Install Rust toolchain + wasm32 target + Trunk"
	@echo "  make setup-python Install Python deps (feedparser, requests, anthropic)"
	@echo ""
	@echo "  make serve        Start dev server — http://localhost:9999 (hot reload)"
	@echo "  make build        Production build → dist/"
	@echo ""
	@echo "  make fetch        Run news fetch (no AI)"
	@echo "  make score        Run article competition (requires ANTHROPIC_API_KEY)"
	@echo "  make synth        Run weekly synthesis (requires ANTHROPIC_API_KEY)"
	@echo "  make score-dry    Run score in dry-run mode (no write)"
	@echo ""
	@echo "  make clean        Remove dist/ and Cargo target/ build artifacts"
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
	$(PYTHON) scripts/synthesize_news.py

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
