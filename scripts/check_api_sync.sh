#!/bin/bash
set -e

API_SPEC_URL="${1:-https://atm-api.local.docker.dev/v3/api-docs/public-mcp-api}"
SPEC_FILE="/tmp/api-spec-sync-check.json"
CLIENT_FILE="shared/src/generated/mcp_client.rs"
MCP_DIR="automoto-user-cli/src"

echo "🔍 Vérification synchronisation API ↔ Client ↔ MCP"
echo "=================================================="
echo ""

# 1. Récupérer spec OpenAPI
echo "📥 Récupération spec OpenAPI..."
curl -s "$API_SPEC_URL" -o "$SPEC_FILE" || {
    echo "❌ Erreur : Impossible récupérer la spec"
    exit 1
}
echo "✅ Spec récupérée"
echo ""

# 2. Extraire endpoints API
echo "📋 Endpoints API détectés:"
ENDPOINTS=$(jq -r '.paths | keys[]' "$SPEC_FILE" | sort)
API_COUNT=$(echo "$ENDPOINTS" | wc -l)
echo "$ENDPOINTS" | head -10
[ "$API_COUNT" -gt 10 ] && echo "   ... et $((API_COUNT - 10)) autres"
echo ""

# 3. Extraire méthodes client générées
echo "📋 Méthodes client générées:"
CLIENT_METHODS=$(grep "pub async fn" "$CLIENT_FILE" | sed 's/.*fn //' | sed 's/(.*//' | sort | uniq)
CLIENT_COUNT=$(echo "$CLIENT_METHODS" | grep -c . || echo 0)
echo "$CLIENT_METHODS" | head -10
[ "$CLIENT_COUNT" -gt 10 ] && echo "   ... et $((CLIENT_COUNT - 10)) autres"
echo ""

# 4. Extraire tools MCP exposés via Python
echo "📋 Tools MCP exposés:"
MCP_TOOLS=$(python3 << 'PYTHON'
import re
import os

tools = set()
for fname in ['tools.rs', 'handlers.rs']:
    fpath = os.path.join('automoto-user-cli/src', fname)
    if not os.path.exists(fpath):
        continue
    with open(fpath) as f:
        content = f.read()
        for match in re.finditer(r'\.add_tool\(\s*"([^"]+)"', content):
            tools.add(match.group(1))

for tool in sorted(tools):
    print(tool)
PYTHON
)
MCP_COUNT=$(echo "$MCP_TOOLS" | grep -c . || echo 0)
echo "$MCP_TOOLS" | head -10
[ "$MCP_COUNT" -gt 10 ] && echo "   ... et $((MCP_COUNT - 10)) autres"
echo ""

# 5. Analyse des divergences
echo "🔍 Analyse des divergences:"
echo ""

# Créer fichiers temporaires pour comparaison
TMP_CLIENT_TOOLS="/tmp/client_tools.txt"
TMP_MCP_TOOLS="/tmp/mcp_tools.txt"

# Convertir noms de méthodes client → tool names (approximatif)
echo "$CLIENT_METHODS" | sed 's/\([A-Z]\)/_\1/g' | sed 's/^_//' | tr '[:upper:]' '[:lower:]' | sort > "$TMP_CLIENT_TOOLS"
echo "$MCP_TOOLS" | sort > "$TMP_MCP_TOOLS"

echo "📊 Comparaison:"
echo "  Endpoints API: $API_COUNT"
echo "  Méthodes client: $CLIENT_COUNT"
echo "  Tools MCP: $MCP_COUNT"
echo ""

# Méthodes client manquantes dans MCP
echo "⚠️  Méthodes client non exposées en MCP:"
NOT_IN_MCP=$(comm -23 "$TMP_CLIENT_TOOLS" "$TMP_MCP_TOOLS" | head -5)
if [ -z "$NOT_IN_MCP" ]; then
    echo "  ✅ Aucune"
else
    echo "$NOT_IN_MCP" | sed 's/^/  - /'
fi
echo ""

# Tools MCP sans correspodance client
echo "⚠️  Tools MCP sans correspodance client:"
NOT_IN_CLIENT=$(comm -13 "$TMP_CLIENT_TOOLS" "$TMP_MCP_TOOLS" | head -5)
if [ -z "$NOT_IN_CLIENT" ]; then
    echo "  ✅ Aucun"
else
    echo "$NOT_IN_CLIENT" | sed 's/^/  - /'
fi
echo ""

# 6. Qualité des schémas MCP (NEW)
echo "🔍 Qualité des schémas MCP tools:"
SCHEMA_ISSUES=$(python3 << 'PYTHON'
import re, sys

try:
    with open('automoto-user-cli/src/tools.rs') as f:
        content = f.read()
except FileNotFoundError:
    print("  ❌ Fichier tools.rs introuvable")
    sys.exit(0)

# Extract each add_tool block (name + schema)
tool_blocks = re.findall(
    r'\.add_tool\(\s*"([^"]+)"\s*,\s*"[^"]*"\s*,\s*json!\((\{.*?\})\s*\)',
    content, re.DOTALL
)

empty_schema = []
no_description = []
for name, schema_str in tool_blocks:
    # Empty properties: "properties": {} or "properties": { }
    if re.search(r'"properties"\s*:\s*\{\s*\}', schema_str):
        empty_schema.append(name)

total = len(tool_blocks)
empty = len(empty_schema)

if empty == 0:
    print(f"  ✅ Tous les {total} tools ont un schéma non-vide")
else:
    print(f"  ⚠️  {empty}/{total} tools ont un schéma vide (non-visibles par les LLMs) :")
    for t in sorted(empty_schema):
        print(f"     - {t}")
PYTHON
)
echo "$SCHEMA_ISSUES"
echo ""

# Résumé
MISSING_IN_MCP=$(comm -23 "$TMP_CLIENT_TOOLS" "$TMP_MCP_TOOLS" | wc -l)
MISSING_IN_CLIENT=$(comm -13 "$TMP_CLIENT_TOOLS" "$TMP_MCP_TOOLS" | wc -l)

if [ "$MISSING_IN_MCP" -eq 0 ] && [ "$MISSING_IN_CLIENT" -eq 0 ]; then
    echo "✅ Synchronisation OK - Pas de divergences majeures"
else
    echo "⚠️  Divergences détectées:"
    echo "  - Méthodes client non exposées: $MISSING_IN_MCP"
    echo "  - Tools MCP sans client: $MISSING_IN_CLIENT"
fi

# Cleanup
rm -f "$TMP_CLIENT_TOOLS" "$TMP_MCP_TOOLS"
