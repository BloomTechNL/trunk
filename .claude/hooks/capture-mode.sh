#!/usr/bin/env bash
set -euo pipefail

STATE_FILE=".claude/tdd_state.env"

# Always clear old state first
rm -f "$STATE_FILE"

# Read the JSON event from stdin and extract the prompt
PROMPT=$(jq -r '.prompt // ""')

# ── red('testName') ────────────────────────────────────────────────
if echo "$PROMPT" | grep -qE "^red\('[^']+'\)"; then
    TARGET=$(echo "$PROMPT" | sed -n "s/^red('\([^']*\)').*/\1/p")
    cat > "$STATE_FILE" << STATE_EOF
MODE=red
TARGET=$TARGET
STATE_EOF
    echo "🔴 TDD RED mode: targeting test '$TARGET'" >&2

# ── green() ────────────────────────────────────────────────────────
elif echo "$PROMPT" | grep -qE "^green\(\)"; then
    cat > "$STATE_FILE" << STATE_EOF
MODE=green
STATE_EOF
    echo "🟢 TDD GREEN mode" >&2

# ── refactor() ─────────────────────────────────────────────────────
elif echo "$PROMPT" | grep -qE "^refactor\(\)"; then
    cat > "$STATE_FILE" << STATE_EOF
MODE=refactor
STATE_EOF
    echo "🔵 TDD REFACTOR mode" >&2
fi

exit 0
