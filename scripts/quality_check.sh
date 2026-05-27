#!/usr/bin/env bash
# Local quality checks — mirrors .github/workflows/quality.yml
# Usage: ./scripts/quality_check.sh [--skip-lhci] [--skip-a11y] [--skip-w3c]
#
# Requires: lhci (npm i -g @lhci/cli), pa11y (npm i -g pa11y)
# Optional: docker (for W3C Nu HTML validation against localhost)

set -euo pipefail

LOCAL_URL="http://localhost:9999"
TRUNK_PID=""
FAILED_JOBS=()

# ── Flags ──────────────────────────────────────────────────────────────────
SKIP_LHCI=false
SKIP_A11Y=false
SKIP_W3C=false
for arg in "$@"; do
  case $arg in
    --skip-lhci) SKIP_LHCI=true ;;
    --skip-a11y) SKIP_A11Y=true ;;
    --skip-w3c)  SKIP_W3C=true  ;;
  esac
done

# ── Colors ─────────────────────────────────────────────────────────────────
GREEN='\033[0;32m'; RED='\033[0;31m'; YELLOW='\033[1;33m'; BOLD='\033[1m'; NC='\033[0m'
ok()   { echo -e "  ${GREEN}✓${NC}  $*"; }
fail() { echo -e "  ${RED}✗${NC}  $*"; }
warn() { echo -e "  ${YELLOW}⚠${NC}  $*"; }
head() { echo -e "\n${BOLD}── $*${NC}"; }

# ── Cleanup ────────────────────────────────────────────────────────────────
cleanup() {
  if [ -n "$TRUNK_PID" ]; then
    echo -e "\nStopping trunk serve (PID $TRUNK_PID)…"
    kill "$TRUNK_PID" 2>/dev/null || true
  fi
}
trap cleanup EXIT INT TERM

# ── Start trunk serve if not already running ───────────────────────────────
head "Site"
if curl -s -o /dev/null -w "%{http_code}" "$LOCAL_URL" 2>/dev/null | grep -q "200"; then
  ok "Site already running at $LOCAL_URL"
else
  echo "  Starting trunk serve…"
  trunk serve > /tmp/trunk-quality.log 2>&1 &
  TRUNK_PID=$!
  READY=false
  for i in $(seq 1 30); do
    STATUS=$(curl -s -o /dev/null -w "%{http_code}" "$LOCAL_URL" 2>/dev/null || echo "0")
    if [ "$STATUS" = "200" ]; then
      ok "Site ready at $LOCAL_URL (${i}×2s)"
      READY=true
      break
    fi
    sleep 2
  done
  if [ "$READY" = "false" ]; then
    fail "Site did not become available after 60s. Check /tmp/trunk-quality.log"
    exit 1
  fi
fi

# ── Lighthouse CI ──────────────────────────────────────────────────────────
if [ "$SKIP_LHCI" = "false" ]; then
  head "Lighthouse CI (perf / a11y / best-practices / SEO)"
  if ! command -v lhci &>/dev/null; then
    warn "lhci not found → npm install -g @lhci/cli"
    FAILED_JOBS+=("lighthouse (not installed)")
  else
    # Collect against local URLs (1 run only for speed), assert with same thresholds
    if lhci collect \
        --url="$LOCAL_URL/" \
        --url="$LOCAL_URL/blog" \
        --url="$LOCAL_URL/veille" \
        --numberOfRuns=1 \
        --settings.onlyCategories=performance,accessibility,best-practices,seo \
      && lhci assert --config=.lighthouserc.json; then
      ok "Lighthouse passed"
    else
      fail "Lighthouse failed — see .lighthouseci/"
      FAILED_JOBS+=("lighthouse")
    fi
  fi
fi

# ── pa11y — WCAG2AA ────────────────────────────────────────────────────────
if [ "$SKIP_A11Y" = "false" ]; then
  head "Accessibility — pa11y WCAG2AA"
  if ! command -v pa11y &>/dev/null; then
    warn "pa11y not found → npm install -g pa11y"
    FAILED_JOBS+=("a11y (not installed)")
  else
    A11Y_FAIL=false
    for path in "/" "/blog" "/veille"; do
      echo -n "  Testing ${LOCAL_URL}${path} … "
      if pa11y --standard WCAG2AA --reporter cli "${LOCAL_URL}${path}" 2>&1; then
        ok "${path}"
      else
        fail "${path}"
        A11Y_FAIL=true
      fi
    done
    [ "$A11Y_FAIL" = "true" ] && FAILED_JOBS+=("a11y")
  fi
fi

# ── W3C Nu HTML Validator (Docker) ─────────────────────────────────────────
if [ "$SKIP_W3C" = "false" ]; then
  head "W3C Nu HTML Validation"
  if ! command -v docker &>/dev/null; then
    warn "Docker not found — W3C validation skipped (runs against live site in CI)"
  else
    echo "  Starting W3C Nu validator container…"
    docker run -d \
      --name w3c-quality-check \
      --network host \
      ghcr.io/validator/validator:latest \
      > /dev/null 2>&1

    # Wait for the validator to boot
    for i in $(seq 1 10); do
      if curl -s -o /dev/null -w "%{http_code}" "http://localhost:8888/" 2>/dev/null | grep -q "200"; then
        break
      fi
      sleep 2
    done

    W3C_FAIL=false
    for path in "/" "/blog" "/veille"; do
      echo -n "  Validating ${LOCAL_URL}${path} … "
      RESULT=$(curl -s "http://localhost:8888/?doc=${LOCAL_URL}${path}&out=json" \
        -H "User-Agent: local-quality-check/bourbask.github.io")
      ERRORS=$(echo "$RESULT" | jq '[.messages[] | select(.type == "error")] | length' 2>/dev/null || echo "0")
      if [ "$ERRORS" = "0" ]; then
        ok "${path}"
      else
        fail "${path} — $ERRORS error(s)"
        echo "$RESULT" | jq -r '.messages[] | select(.type == "error") | "     [\(.type)] \(.message) (line \(.lastLine // "?"))"' 2>/dev/null || true
        W3C_FAIL=true
      fi
    done

    docker stop w3c-quality-check > /dev/null 2>&1
    docker rm   w3c-quality-check > /dev/null 2>&1

    [ "$W3C_FAIL" = "true" ] && FAILED_JOBS+=("html-validation")
  fi
fi

# ── Note: security headers ─────────────────────────────────────────────────
head "Security headers"
warn "Trunk (dev server) ne sert pas les mêmes headers que GitHub Pages."
warn "Ce check ne tourne qu'en CI contre le site live."

# ── Summary ────────────────────────────────────────────────────────────────
echo -e "\n${BOLD}══ Summary ══${NC}"
if [ ${#FAILED_JOBS[@]} -eq 0 ]; then
  echo -e "${GREEN}All checks passed.${NC}"
  exit 0
else
  echo -e "${RED}Failed:${NC}"
  for job in "${FAILED_JOBS[@]}"; do
    echo "  - $job"
  done
  exit 1
fi
