#!/bin/bash

# Script d'installation de annuaire-cli pour l'utilisateur courant
# Usage: ./scripts/deploy_locally.sh

set -e

WORKSPACE_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
LOCAL_BIN="$HOME/.local/bin"
RELEASE_DIR="$WORKSPACE_ROOT/target/release"

echo "🔨 Installation de annuaire-cli..."
echo ""

echo "📦 Compilation en release..."
cd "$WORKSPACE_ROOT"
cargo build --release 2>&1 | grep -E "(Compiling|Finished|error)" || true

echo "📁 Création de $LOCAL_BIN..."
mkdir -p "$LOCAL_BIN"

echo "📋 Installation du binaire..."
cp "$RELEASE_DIR/annuaire-cli" "$LOCAL_BIN/"
chmod +x "$LOCAL_BIN/annuaire-cli"

if ! echo "$PATH" | grep -q "$LOCAL_BIN"; then
    echo "🔧 Ajout de $LOCAL_BIN au PATH..."
    if [ -n "$ZSH_VERSION" ]; then
        RC_FILE="$HOME/.zshrc"
    else
        RC_FILE="$HOME/.bashrc"
    fi
    if ! grep -q "export PATH.*\.local/bin" "$RC_FILE" 2>/dev/null; then
        echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$RC_FILE"
        echo "✅ Ajouté à $RC_FILE"
    fi
    export PATH="$HOME/.local/bin:$PATH"
fi

echo ""
echo "✨ Installation terminée!"
echo ""
echo "📊 Vérification:"
echo "  annuaire-cli: $(which annuaire-cli)"
echo ""

EXPECTED_VERSION="$(cat "$WORKSPACE_ROOT/VERSION")"
INSTALLED_VERSION="$("$LOCAL_BIN/annuaire-cli" --version | awk '{print $2}')"

if [ "$INSTALLED_VERSION" != "$EXPECTED_VERSION" ]; then
    echo "❌ Version installée ($INSTALLED_VERSION) ≠ VERSION ($EXPECTED_VERSION)"
    exit 1
fi
echo "✅ Version vérifiée: $INSTALLED_VERSION"
echo ""

echo "🚀 Commandes disponibles:"
annuaire-cli --help | head -5
