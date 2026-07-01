#!/usr/bin/env bash
set -euo pipefail

STATE_FILE=".claude/tdd_state.env"
HOOKS_DIR="$(cd "$(dirname "$0")" && pwd)"

# No state file → nothing to enforce, exit cleanly
if [ ! -f "$STATE_FILE" ]; then
    exit 0
fi

# Source the state and delete it immediately so it doesn't linger
# shellcheck disable=SC1090
source "$STATE_FILE"
rm -f "$STATE_FILE"

# Export variables so sub-scripts can read them
export MODE
export TARGET

# Delegate to the appropriate mode script
case "$MODE" in
    red)      exec "$HOOKS_DIR/red.sh" ;;
    green)    exec "$HOOKS_DIR/green.sh" ;;
    refactor) exec "$HOOKS_DIR/refactor.sh" ;;
    *)
        echo "⚠️  Unknown TDD mode '$MODE' — skipping verification." >&2
        exit 0
        ;;
esac
