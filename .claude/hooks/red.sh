#!/usr/bin/env bash
set -euo pipefail


echo "🔴 TDD RED: targeting test '$TARGET'" >&2

scripts/check_acceptance_mods.sh

# ── Gate 1: build must succeed ────────────────────────────────────
echo "  Building..." >&2
if ! cargo build 2>&1; then
    echo "" >&2
    echo "❌ RED GATE FAILED: cargo build failed." >&2
    echo "   Fix compilation errors before proceeding." >&2
    exit 2
fi

# ── Gate 2: the targeted test must fail ───────────────────────────
echo "  Running test '$TARGET'..." >&2
if cargo test -q "$TARGET" 2>&1; then
    echo "" >&2
    echo "❌ RED GATE FAILED: Test '$TARGET' passed." >&2
    echo "   You are in RED mode — write a failing test first." >&2
    exit 2
fi

echo "✅ RED GATE PASSED: Test '$TARGET' is correctly failing." >&2
