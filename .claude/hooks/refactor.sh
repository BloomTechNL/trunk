#!/usr/bin/env bash
set -euo pipefail

SENSOR_DIR="$(cd "$(dirname "$0")/../../agent_sensors/refactor" && pwd)"

echo "🔵 TDD REFACTOR: verifying lint baseline, formatting, and tests..." >&2

for sensor in "$SENSOR_DIR"/*.sh; do
    [ -x "$sensor" ] || continue
    if ! "$sensor"; then
        echo "❌ REFACTOR GATE FAILED" >&2
        exit 2
    fi
done

echo "✅ REFACTOR GATE PASSED: Lints stable, formatting clean, and all tests pass." >&2
