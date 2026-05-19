// ⚠️  AUTO-GENERATED - NE PAS MODIFIER MANUELLEMENT
// Generated from OpenAPI spec
// Regenerate: ./scripts/generate-client.sh

use serde::{Serialize, Deserialize};

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
    pub tags: Vec<String>,
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HTTPValidationError {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<Vec<ValidationError>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationError {
    pub loc: Vec<Option<serde_json::Value>>,
    pub msg: String,
    #[serde(rename = "type")]
    pub r#type: String,
}

