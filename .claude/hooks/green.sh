#!/usr/bin/env bash
set -euo pipefail

echo "🟢 TDD GREEN: verifying all tests pass..." >&2

scripts/check_acceptance_mods.sh

if cargo test -q 2>&1; then
    echo "✅ GREEN GATE PASSED: All tests pass." >&2
else
    echo "" >&2
    echo "❌ GREEN GATE FAILED: Tests are failing." >&2
    echo "   Make all tests pass before the turn ends." >&2
    exit 2
fi
