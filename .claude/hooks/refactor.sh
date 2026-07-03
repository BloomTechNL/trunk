#!/usr/bin/env bash
set -euo pipefail

echo "🔵 TDD REFACTOR: verifying lint baseline, formatting, and tests..." >&2

echo "  Checking clippy ratchet..." >&2
if ! python3 scripts/clippy_ratchet.py; then
    echo "" >&2
    echo "❌ REFACTOR GATE FAILED: New clippy lints introduced or compilation broken." >&2
    exit 2
fi

echo "  Running cargo fmt..." >&2
if ! cargo fmt -- --check 2>&1; then
    echo "" >&2
    echo "❌ REFACTOR GATE FAILED: cargo fmt found unformatted code." >&2
    echo "   Run 'cargo fmt' to fix formatting before the turn ends." >&2
    exit 2
fi

echo " Checking that acceptance tests live where they belong"
scripts/check_acceptance_mods.sh

echo "  Running tests..." >&2
if ! cargo test -q 2>&1; then
    echo "" >&2
    echo "❌ REFACTOR GATE FAILED: Tests are failing." >&2
    echo "   Keep all tests green while refactoring." >&2
    exit 2
fi

echo "✅ REFACTOR GATE PASSED: Lints stable, formatting clean, and all tests pass." >&2