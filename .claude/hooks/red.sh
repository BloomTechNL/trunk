#!/usr/bin/env bash
set -euo pipefail

SENSOR_DIR="$(cd "$(dirname "$0")/../../agent_sensors/red" && pwd)"

echo "🔴 TDD RED: targeting test '$TARGET'" >&2

for sensor in "$SENSOR_DIR"/*.sh; do
    [ -x "$sensor" ] || continue
    if ! "$sensor"; then
        echo "❌ RED GATE FAILED" >&2
        exit 2
    fi
done

echo "✅ RED GATE PASSED: Test '$TARGET' is correctly failing." >&2
