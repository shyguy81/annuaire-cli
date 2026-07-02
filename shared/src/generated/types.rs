// ⚠️  AUTO-GENERATED - NE PAS MODIFIER MANUELLEMENT
// Generated from OpenAPI spec
// Regenerate: ./scripts/generate-client.sh

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActionStatus {
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ActionType {
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BusinessPotential {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactCreate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adresse: Option<String>,
    pub email: String,
    pub nom: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organisation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adresse: Option<String>,
    pub created_at: String,
    pub email: String,
    pub id: String,
    pub nom: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organisation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContactUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adresse: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nom: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organisation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardResponse {
    pub active_relations: i64,
    pub due_today: i64,
    pub high_potential: i64,
    pub overdue: i64,
    pub recent_interactions: i64,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HTTPValidationError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<Vec<ValidationError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionCreate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interaction_date: Option<String>,
    pub interaction_type: InteractionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InteractionResponse {
    pub contact_id: String,
    pub created_at: String,
    pub id: String,
    pub interaction_date: String,
    pub interaction_type: InteractionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InteractionType {
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Priority {
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProximityLevel {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipActionCreate {
    pub action_type: ActionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    pub priority: Priority,
    pub status: ActionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipActionResponse {
    pub action_type: ActionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    pub contact_id: String,
    pub created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    pub id: String,
    pub priority: Priority,
    pub status: ActionStatus,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RelationshipActionUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<ActionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<Priority>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ActionStatus>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipProfileCreate {
    pub business_potential: BusinessPotential,
    pub proximity_level: ProximityLevel,
    pub relationship_type: RelationshipType,
    pub trust_level: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelationshipProfileResponse {
    pub business_potential: BusinessPotential,
    pub contact_id: String,
    pub created_at: String,
    pub id: String,
    pub proximity_level: ProximityLevel,
    pub relationship_type: RelationshipType,
    pub trust_level: i64,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RelationshipProfileUpdate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_potential: Option<BusinessPotential>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proximity_level: Option<ProximityLevel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relationship_type: Option<RelationshipType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trust_level: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RelationshipType {
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub loc: Vec<Option<serde_json::Value>>,
    pub msg: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

