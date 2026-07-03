#!/usr/bin/env bash
set -euo pipefail

echo "  Running tests..." >&2
if ! cargo test -q 2>&1; then
    echo "  ❌ Tests are failing." >&2
    exit 2
fi
