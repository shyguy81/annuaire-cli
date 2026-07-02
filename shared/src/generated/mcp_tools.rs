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
            "description": "Rechercher des contacts par nom ou email",
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
            "name": "list_interactions_contacts__contact_id__interactions_get",
            "description": "List all interactions for a contact with optional filtering and pagination.\n\nReturns a paginated list of interaction records. Supports filtering by interaction type and date range.\n\n**Parameters:**\n- contact_id: UUID of the contact\n- skip: Number of records to skip (default 0)\n- limit: Maximum records to return (default 100, max 1000)\n- type: Filter by interaction type (call, email, meeting, message, other)\n- since: Filter interactions on or after date (format: YYYY-MM-DD)\n\n**Responses:**\n- 200: Interactions list returned with pagination metadata\n- 404: Contact not found\n- 400: Invalid filter values",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        }),
        json!({
            "name": "create_interaction_contacts__contact_id__interactions_post",
            "description": "Create a new interaction record for a contact.\n\nRecords an interaction (call, email, meeting, message, or other) with a contact. Useful for tracking communication history.\n\n**Parameters:**\n- contact_id: UUID of the contact\n- interaction: Interaction data (type is required, date defaults to now if not provided)\n\n**Responses:**\n- 201: Interaction created successfully\n- 404: Contact not found",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        }),
        json!({
            "name": "create_relationship_action_contacts__contact_id__relationship_actions_post",
            "description": "Create a new relationship action (task/reminder) for a contact.\n\nActions represent tasks or reminders to be completed with a contact. They have status, priority, and optional due date.\n\n**Parameters:**\n- contact_id: UUID of the contact\n- action: Action data (type, priority, status, optional due_date)\n\n**Responses:**\n- 201: Action created successfully\n- 404: Contact not found",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        }),
        json!({
            "name": "get_relationship_profile_contacts__contact_id__relationship_profile_get",
            "description": "Retrieve the relationship profile for a contact.\n\nEach contact can have at most one relationship profile. This endpoint retrieves all relationship metadata.\n\n**Parameters:**\n- contact_id: UUID of the contact\n\n**Responses:**\n- 200: Profile found and returned\n- 404: Contact not found or has no profile",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        }),
        json!({
            "name": "update_relationship_profile_contacts__contact_id__relationship_profile_patch",
            "description": "Update the relationship profile for a contact.\n\nAll fields are optional. Only provided fields will be updated. This allows partial updates to the profile.\n\n**Parameters:**\n- contact_id: UUID of the contact\n- profile_update: Fields to update (all optional)\n\n**Responses:**\n- 200: Profile updated successfully\n- 404: Contact not found or has no profile\n- 400: Invalid field values (e.g., invalid enum values)",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        }),
        json!({
            "name": "create_relationship_profile_contacts__contact_id__relationship_profile_post",
            "description": "Create a relationship profile for a contact.\n\nA contact can have at most one relationship profile. This endpoint stores relationship metadata including:\n- Relationship type (spouse, family, business, mentor, friend, acquaintance)\n- Proximity level (cold, warm, active, close) - how close/engaged the relationship is\n- Trust level (numeric) - trust score for the contact\n- Business potential (low, medium, high) - business opportunity assessment\n\n**Parameters:**\n- contact_id: UUID of the contact (must exist)\n- profile: Relationship profile data\n\n**Responses:**\n- 201: Profile created successfully\n- 404: Contact not found\n- 409: Contact already has a relationship profile",
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
        json!({
            "name": "get_dashboard_rap_dashboard_get",
            "description": "Get RAP system dashboard with aggregated metrics.\n\nReturns system-wide statistics about relationship actions, profiles, and interactions.\nAll metrics are calculated in real-time using optimized SQL aggregations.\n\n**Metrics returned:**\n- due_today: Count of actions with 'todo' or 'in_progress' status due today\n- overdue: Count of actions with 'todo' or 'in_progress' status due before today\n- active_relations: Count of distinct contacts with 'warm', 'active', or 'close' proximity levels\n- high_potential: Count of distinct contacts with 'high' business potential\n- recent_interactions: Count of interactions from the last 7 days\n- timestamp: Current time in ISO 8601 format (UTC)\n\n**Responses:**\n- 200: Dashboard metrics returned successfully",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        }),
        json!({
            "name": "list_relationship_actions_relationship_actions_get",
            "description": "List all relationship actions with optional filtering and pagination.\n\nReturns a paginated list of all actions. Supports filtering by status, priority, and/or contact.\n\n**Parameters:**\n- skip: Number of records to skip (default 0)\n- limit: Maximum records to return (default 100, max 1000)\n- status: Filter by status (todo, in_progress, completed, cancelled)\n- priority: Filter by priority (low, medium, high)\n- contact_id: Filter by contact UUID\n\n**Responses:**\n- 200: Actions list returned with pagination metadata\n- 400: Invalid filter values",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        }),
        json!({
            "name": "list_due_relationship_actions_relationship_actions_due_get",
            "description": "List actions that are due today or overdue.\n\nReturns actions with status 'todo' or 'in_progress' that have a due date on or before today.\nIncludes metadata about due_today and overdue counts.\n\n**Parameters:**\n- skip: Number of records to skip (default 0)\n- limit: Maximum records to return (default 100, max 1000)\n\n**Responses:**\n- 200: List of due/overdue actions with detailed count information",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        }),
        json!({
            "name": "update_relationship_action_relationship_actions__action_id__patch",
            "description": "Update a relationship action.\n\nAll fields are optional. Only provided fields will be updated. Allows partial updates.\n\n**Parameters:**\n- action_id: UUID of the action to update\n- action_update: Fields to update (all optional)\n\n**Responses:**\n- 200: Action updated successfully\n- 404: Action not found\n- 400: Invalid field values",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        }),
        json!({
            "name": "complete_relationship_action_relationship_actions__action_id__complete_patch",
            "description": "Mark a relationship action as completed.\n\nSets the action status to 'completed' and records the completion timestamp.\n\n**Parameters:**\n- action_id: UUID of the action to mark complete\n\n**Responses:**\n- 200: Action marked as completed\n- 404: Action not found",
            "inputSchema": {
                "type": "object",
                "properties": {}
            }
        }),
    ]
}
