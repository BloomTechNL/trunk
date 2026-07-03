#!/usr/bin/env bash
set -euo pipefail

echo "  Checking formatting..." >&2
if ! cargo fmt -- --check 2>&1; then
    echo "  ❌ Unformatted code found. Run 'cargo fmt' to fix." >&2
    exit 2
fi
