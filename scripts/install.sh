#!/usr/bin/env bash
set -e

ORG="BloomTechNL"
REPO="technical-goodiebag"
BINARY_NAME="g"
INSTALL_DIR="$HOME/.local/bin"

echo "======================================"
echo "  Installing / Updating $BINARY_NAME"
echo "======================================"

# --- 1. Token Authentication ---
echo "Please enter your GitHub Personal Access Token (requires 'repo' scope):"
read -s GITHUB_TOKEN
echo ""

# --- 2. Fetch Latest Release Asset ID ---
echo "Locating the latest release..."
API_RESPONSE=$(curl -s -H "Authorization: Bearer $GITHUB_TOKEN" "https://api.github.com/repos/$ORG/$REPO/releases/tags/latest")

# Extract the API URL for the asset download
ASSET_URL=$(echo "$API_RESPONSE" | grep -o "https://api.github.com/repos/$ORG/$REPO/releases/assets/[0-9]*" | head -n 1)

if [ -z "$ASSET_URL" ]; then
    echo "❌ Error: Could not find the release asset."
    exit 1
fi

# --- 3. Download the Binary ---
echo "Downloading $BINARY_NAME..."
mkdir -p "$INSTALL_DIR"

# Download using the API octet-stream header
curl -sL -f \
    -H "Authorization: Bearer $GITHUB_TOKEN" \
    -H "Accept: application/octet-stream" \
    "$ASSET_URL" \
    -o "$INSTALL_DIR/$BINARY_NAME"

# --- 4. Execution & Quarantine Cleanup ---
echo "Configuring permissions..."
chmod +x "$INSTALL_DIR/$BINARY_NAME"

echo "Bypassing Apple Gatekeeper..."
xattr -d com.apple.quarantine "$INSTALL_DIR/$BINARY_NAME" 2>/dev/null || true

# --- 5. PATH Verification ---
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo ""
    echo "⚠️  NOTE: $INSTALL_DIR is not in your PATH environment variable."
    echo "To run the tool globally, add this line to your ~/.zshrc:"
    echo "export PATH=\"\$PATH:$INSTALL_DIR\""
fi

echo ""
echo "✅ Success! '$BINARY_NAME' is ready to use."
