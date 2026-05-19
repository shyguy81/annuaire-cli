#!/bin/bash
set -e

cd "$(dirname "$0")/.."

# Show usage if --help
if [[ "$1" == "--help" || "$1" == "-h" ]]; then
    echo "🔨 Générateur de clients API OpenAPI → Rust"
    echo ""
    echo "Usage: ./scripts/generate-client.sh [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  (aucun)         Complet : télécharger specs + générer clients Rust"
    echo "  --fetch-only    Télécharger uniquement les specs OpenAPI"
    echo "  --generate-only Générer code Rust uniquement (à partir des specs existantes)"
    echo "  --help          Afficher cette aide"
    echo ""
    echo "Exemples:"
    echo "  ./scripts/generate-client.sh"
    echo "  ./scripts/generate-client.sh --fetch-only"
    echo "  ./scripts/generate-client.sh --generate-only"
    echo ""
    exit 0
fi

echo "🔨 Génération des clients API Annuaire"
echo ""
if [[ "$1" == "--fetch-only" ]]; then
    echo "  Mode : Téléchargement des specs uniquement"
elif [[ "$1" == "--generate-only" ]]; then
    echo "  Mode : Génération à partir des specs existantes"
else
    echo "  Mode : Complet (téléchargement + génération)"
fi
echo ""

cargo run -p generate-client --quiet -- "$@"

echo ""
echo "✅ Génération terminée"
echo ""
echo "📂 Fichiers générés :"
echo "   • shared/src/generated/client.rs"
echo "   • shared/src/generated/types.rs"
echo "   • shared/src/generated/mcp_tools.rs"
echo "   • shared/src/generated/mod.rs"
echo ""
echo "📋 Specs téléchargées :"
echo "   • .specs/annuaire-api.json"
echo ""
echo "💡 Workflow recommandé :"
echo "   1. ./scripts/generate-client.sh --fetch-only"
echo "   2. ./scripts/generate-client.sh --generate-only"
