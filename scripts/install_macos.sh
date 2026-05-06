#!/usr/bin/env bash
set -euo pipefail

# ---------- check system ----------
if [[ "$(uname -s)" != "Darwin" ]]; then
    echo "This installer is only for macOS (Darwin)."
    exit 1
fi

ARCH=$(uname -m)
if [[ "$ARCH" != "arm64" ]]; then
    echo "Only Apple Silicon (arm64) is currently supported. Got: $ARCH"
    exit 1
fi

# ---------- download binary ----------
RELEASE_URL="https://github.com/BloomTechNL/trunk/releases/latest/download/g"
TMP_BIN="/tmp/trunk-g-$(date +%s)"

echo "Downloading latest g binary …"
curl -L --fail --progress-bar -o "$TMP_BIN" "$RELEASE_URL"

# ---------- prepare binary ----------
chmod +x "$TMP_BIN"

# Remove quarantine flag (macOS may add it to downloaded files)
xattr -d com.apple.quarantine "$TMP_BIN" 2>/dev/null || true

# ---------- choose install directory ----------
TARGET_DIR=""
if [[ -d "$HOME/.local/bin" ]] && [[ ":$PATH:" == *":$HOME/.local/bin:"* ]]; then
    TARGET_DIR="$HOME/.local/bin"
elif [[ -d "/usr/local/bin" ]] && [[ -w "/usr/local/bin" ]]; then
    TARGET_DIR="/usr/local/bin"
else
    TARGET_DIR="$HOME/bin"
    mkdir -p "$TARGET_DIR"
    echo "Note: adding $TARGET_DIR to your PATH is recommended."
fi

DEST="$TARGET_DIR/g"
mv -f "$TMP_BIN" "$DEST"
echo "Installed to $DEST"

# ---------- verify ----------
if "$DEST" --version &>/dev/null; then
    echo "Success! $( "$DEST" --version )"
else
    echo "Installation succeeded, but 'g --version' failed. Check your binary."
    exit 1
fi

# ---------- friendly reminder ----------
if alias g &>/dev/null; then
    echo "NOTE: You have an existing shell alias for 'g' (e.g. from OhMyZsh git plugin)."
    echo "Run 'unalias g' if you want to use the trunk binary by default."
fi