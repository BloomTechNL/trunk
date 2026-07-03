#!/usr/bin/env bash
set -euo pipefail

echo "  Building..." >&2
if ! cargo build 2>&1; then
    echo "  ❌ Compilation failed." >&2
    exit 2
fi
