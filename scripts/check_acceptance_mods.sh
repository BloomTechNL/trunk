#!/usr/bin/env bash
set -euo pipefail

for file in tests/acceptance/*.rs; do
    [[ "$(basename "$file")" == "mod.rs" ]] && continue
    mod_name="$(basename "$file" .rs)"
    if ! grep -q "mod $mod_name;" tests/acceptance_tests.rs; then
        echo "ERROR: $file is not declared in tests/acceptance_tests.rs" >&2
        echo "Add: mod $mod_name;" >&2
        exit 2
    fi
done
