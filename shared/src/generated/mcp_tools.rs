// ⚠️  AUTO-GENERATED - NE PAS MODIFIER MANUELLEMENT
// Generated from OpenAPI spec
// Regenerate: ./scripts/generate-client.sh

use serde_json::{json, Value};

/// Returns all MCP tool definitions for the Annuaire API.
pub fn get_tool_definitions() -> Vec<Value> {
    vec![
        json!({
            "name": "list_contacts_contacts_get",
            "description": "Lister tous les contacts",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        }),
        json!({
            "name": "create_contact_contacts_post",
            "description": "Créer un nouveau contact",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        }),
        json!({
            "name": "search_contacts_contacts_search__query__get",
            "description": "Chercher contacts par nom/email/organisation",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        }),
        json!({
            "name": "delete_contact_contacts__contact_id__delete",
            "description": "Supprimer un contact",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        }),
        json!({
            "name": "get_contact_contacts__contact_id__get",
            "description": "Récupérer un contact par ID",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        }),
        json!({
            "name": "update_contact_contacts__contact_id__put",
            "description": "Mettre à jour un contact",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        }),
        json!({
            "name": "health_health_get",
            "description": "Health check",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        }),
    ]
}
