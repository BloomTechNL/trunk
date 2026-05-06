#!/usr/bin/env bash
set -euo pipefail

INSTALL_DIR="$HOME/.local/bin"
TMP_BIN="/tmp/trunk-g-$(date +%s)"
DOWNLOAD_URL="https://github.com/BloomTechNL/trunk/releases/latest/download/g"

check_architecture() {
    if [[ "$(uname -s)" != "Darwin" ]]; then
        echo "Only macOS is supported." >&2
        exit 1
    fi
    if [[ "$(uname -m)" != "arm64" ]]; then
        echo "Only Apple Silicon (arm64) is supported." >&2
        exit 1
    fi
}

download_binary() {
    local url="$1"
    local dest="$2"

    echo "Downloading latest g …"
    curl -L --fail --progress-bar -o "$dest" "$url"
    chmod +x "$dest"
    xattr -d com.apple.quarantine "$dest" 2>/dev/null || true
}

move_to_final_dest() {
    local tmp="$1"
    local install_dir="$2"

    mkdir -p "$install_dir"
    local final_path="$install_dir/g"
    mv -f "$tmp" "$final_path"
    echo "Installed to $final_path"
}

append_to_path() {
    local dir="$1"
    local export_line="export PATH=\"$dir:\$PATH\""

    for f in "$HOME/.zshrc" "$HOME/.bash_profile" "$HOME/.bashrc"; do
        if [ -f "$f" ]; then
            if ! grep -qF "$dir" "$f" 2>/dev/null; then
                echo "$export_line" >> "$f"
            fi
        fi
    done
}

verify_binary() {
    local bin_path="$1"

    if "$bin_path" --version &>/dev/null; then
        echo "Success! $( "$bin_path" --version )"
    else
        echo "Installation failed: 'g --version' failed." >&2
        exit 1
    fi
}

warn_for_alias() {
    if alias g &>/dev/null; then
        echo "NOTE: You have an alias for 'g' (OhMyZsh git plugin). Run 'unalias g' to use trunk."
    fi
}

check_architecture
download_binary "$DOWNLOAD_URL" "$TMP_BIN"
move_to_final_dest "$TMP_BIN" "$INSTALL_DIR"
append_to_path "$INSTALL_DIR"
verify_binary "$INSTALL_DIR/g"
warn_for_alias

echo ""
echo "Installation complete. To start using 'g', please restart your terminal"
echo "or run: exec $SHELL -l"
