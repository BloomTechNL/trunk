#!/usr/bin/env bash
set -euo pipefail

SENSOR_DIR="$(cd "$(dirname "$0")/../../agent_sensors/green" && pwd)"

echo "🟢 TDD GREEN: verifying all tests pass..." >&2

for sensor in "$SENSOR_DIR"/*.sh; do
    [ -x "$sensor" ] || continue
    if ! "$sensor"; then
        echo "❌ GREEN GATE FAILED" >&2
        exit 2
    fi
done

echo "✅ GREEN GATE PASSED: All tests pass." >&2
