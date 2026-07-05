#!/usr/bin/env bash
set -euo pipefail

TOOL_CALL=$(cat)

TOOL_NAME=$(python3 -c "import sys,json; d=json.load(sys.stdin); print(d.get('tool_name',''))" <<< "$TOOL_CALL")
TOOL_INPUT=$(python3 -c "import sys,json; d=json.load(sys.stdin); print(d.get('tool_input',''))" <<< "$TOOL_CALL")

block() {
    echo "Blocked: tdd_state.env is managed by TDD hooks and must not be modified directly." >&2
    exit 2
}

case "$TOOL_NAME" in
    Write|Edit|NotebookEdit)
        TARGET=$(python3 -c "
import sys, json
d = json.loads(sys.stdin.read())
print(d.get('file_path', '') + d.get('notebook_path', ''))
" <<< "$TOOL_INPUT")
        if echo "$TARGET" | grep -q 'tdd_state\.env'; then
            block
        fi
        ;;
    Bash)
        CMD=$(python3 -c "import sys,json; d=json.loads(sys.stdin.read()); print(d.get('command',''))" <<< "$TOOL_INPUT")
        if echo "$CMD" | grep -q 'tdd_state\.env'; then
            block
        fi
        ;;
esac

exit 0
