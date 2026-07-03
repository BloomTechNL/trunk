#!/usr/bin/env bash
set -euo pipefail

echo "  Running test '$TARGET'..." >&2
if cargo test -q "$TARGET" 2>&1; then
    echo "  ❌ Test '$TARGET' passed — it should have failed." >&2
    echo "  You are in RED mode — write a failing test first." >&2
    exit 2
fi
