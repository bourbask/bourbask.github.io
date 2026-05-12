#!/usr/bin/env bash
# Take visual screenshots at all breakpoints.
# Requires: trunk serve running on http://localhost:8080
# Usage: ./scripts/screenshot.sh [output-dir]

set -e

# Find the npx-cached playwright node_modules
NP=$(find ~/.npm/_npx -maxdepth 3 -name "playwright" -type d 2>/dev/null | head -1)
if [ -z "$NP" ]; then
  echo "playwright not found in npx cache — running 'npx playwright --version' to cache it"
  npx --yes playwright --version > /dev/null
  NP=$(find ~/.npm/_npx -maxdepth 3 -name "playwright" -type d 2>/dev/null | head -1)
fi

export NODE_PATH="$(dirname "$NP")"
echo "Using playwright at: $NP"

node "$(dirname "$0")/screenshot.js" "${1:-screenshots}"
