#!/usr/bin/env bash
set -euo pipefail

echo "🔵 TDD REFACTOR: verifying formatting and tests..." >&2

# ── Gate 1: formatting must be clean ──────────────────────────────
echo "  Running cargo fmt..." >&2
if ! cargo fmt -- --check 2>&1; then
    echo "" >&2
    echo "❌ REFACTOR GATE FAILED: cargo fmt found unformatted code." >&2
    echo "   Run 'cargo fmt' to fix formatting before the turn ends." >&2
    exit 2
fi

# ── Gate 2: all tests must pass ───────────────────────────────────
echo "  Running tests..." >&2
if ! cargo test -q 2>&1; then
    echo "" >&2
    echo "❌ REFACTOR GATE FAILED: Tests are failing." >&2
    echo "   Keep all tests green while refactoring." >&2
    exit 2
fi

echo "✅ REFACTOR GATE PASSED: Formatting clean and all tests pass." >&2
