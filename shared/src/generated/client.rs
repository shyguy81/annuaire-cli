// ⚠️  AUTO-GENERATED - NE PAS MODIFIER MANUELLEMENT
// Generated from OpenAPI spec
// Regenerate: ./scripts/generate-client.sh

use reqwest::Client as HttpClient;
use anyhow::{Result, anyhow};
use super::types::*;

pub struct ApiClient {
    base_url: String,
    client: HttpClient,
}

impl ApiClient {
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            base_url: base_url.into(),
            client: HttpClient::new(),
        }
    }

    async fn check_response(response: reqwest::Response) -> Result<reqwest::Response> {
        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(anyhow!("HTTP {}: {}", status, body));
        }
        Ok(response)
    }

    pub async fn list_contacts(&self, skip: Option<i64>, limit: Option<i64>) -> Result<Vec<ContactResponse>> {
        let url = format!("{base_url}/contacts", base_url = self.base_url);
        let req = self.client.get(&url);
        let req = if let Some(v) = skip { req.query(&[("skip", v.to_string())]) } else { req };
        let req = if let Some(v) = limit { req.query(&[("limit", v.to_string())]) } else { req };
        let response = Self::check_response(req.send().await?).await?;
        Ok(response.json().await?)
    }

    pub async fn create_contact(&self, body: &ContactCreate) -> Result<ContactResponse> {
        let url = format!("{base_url}/contacts", base_url = self.base_url);
        let req = self.client.post(&url);
        let req = req.json(body);
        let response = Self::check_response(req.send().await?).await?;
        Ok(response.json().await?)
    }

    pub async fn search_contacts(&self, query: &str) -> Result<Vec<ContactResponse>> {
        let url = format!("{base_url}/contacts/search/{query}", base_url = self.base_url, query = query);
        let req = self.client.get(&url);
        let response = Self::check_response(req.send().await?).await?;
        Ok(response.json().await?)
    }

    pub async fn delete_contact(&self, contact_id: &str) -> Result<()> {
        let url = format!("{base_url}/contacts/{contact_id}", base_url = self.base_url, contact_id = contact_id);
        let req = self.client.delete(&url);
        let response = Self::check_response(req.send().await?).await?;
        Ok(())
    }

    pub async fn get_contact(&self, contact_id: &str) -> Result<ContactResponse> {
        let url = format!("{base_url}/contacts/{contact_id}", base_url = self.base_url, contact_id = contact_id);
        let req = self.client.get(&url);
        let response = Self::check_response(req.send().await?).await?;
        Ok(response.json().await?)
    }

    pub async fn update_contact(&self, contact_id: &str, body: &ContactUpdate) -> Result<ContactResponse> {
        let url = format!("{base_url}/contacts/{contact_id}", base_url = self.base_url, contact_id = contact_id);
        let req = self.client.put(&url);
        let req = req.json(body);
        let response = Self::check_response(req.send().await?).await?;
        Ok(response.json().await?)
    }

    pub async fn list_interactions(&self, contact_id: &str, skip: Option<i64>, limit: Option<i64>, r#type: Option<String>, since: Option<String>) -> Result<serde_json::Value> {
        let url = format!("{base_url}/contacts/{contact_id}/interactions", base_url = self.base_url, contact_id = contact_id);
        let req = self.client.get(&url);
        let req = if let Some(v) = skip { req.query(&[("skip", v.to_string())]) } else { req };
        let req = if let Some(v) = limit { req.query(&[("limit", v.to_string())]) } else { req };
        let req = if let Some(v) = r#type { req.query(&[("type", v.to_string())]) } else { req };
        let req = if let Some(v) = since { req.query(&[("since", v.to_string())]) } else { req };
        let response = Self::check_response(req.send().await?).await?;
        Ok(response.json().await?)
    }

    pub async fn create_interaction(&self, contact_id: &str, body: &InteractionCreate) -> Result<InteractionResponse> {
        let url = format!("{base_url}/contacts/{contact_id}/interactions", base_url = self.base_url, contact_id = contact_id);
        let req = self.client.post(&url);
        let req = req.json(body);
        let response = Self::check_response(req.send().await?).await?;
        Ok(response.json().await?)
    }

    pub async fn create_relationship_action(&self, contact_id: &str, body: &RelationshipActionCreate) -> Result<RelationshipActionResponse> {
        let url = format!("{base_url}/contacts/{contact_id}/relationship-actions", base_url = self.base_url, contact_id = contact_id);
        let req = self.client.post(&url);
        let req = req.json(body);
        let response = Self::check_response(req.send().await?).await?;
        Ok(response.json().await?)
    }

    pub async fn get_relationship_profile(&self, contact_id: &str) -> Result<RelationshipProfileResponse> {
        let url = format!("{base_url}/contacts/{contact_id}/relationship-profile", base_url = self.base_url, contact_id = contact_id);
        let req = self.client.get(&url);
        let response = Self::check_response(req.send().await?).await?;
        Ok(response.json().await?)
    }

    pub async fn update_relationship_profile(&self, contact_id: &str, body: &RelationshipProfileUpdate) -> Result<RelationshipProfileResponse> {
        let url = format!("{base_url}/contacts/{contact_id}/relationship-profile", base_url = self.base_url, contact_id = contact_id);
        let req = self.client.patch(&url);
        let req = req.json(body);
        let response = Self::check_response(req.send().await?).await?;
        Ok(response.json().await?)
    }

    pub async fn create_relationship_profile(&self, contact_id: &str, body: &RelationshipProfileCreate) -> Result<RelationshipProfileResponse> {
        let url = format!("{base_url}/contacts/{contact_id}/relationship-profile", base_url = self.base_url, contact_id = contact_id);
        let req = self.client.post(&url);
        let req = req.json(body);
        let response = Self::check_response(req.send().await?).await?;
        Ok(response.json().await?)
    }

    pub async fn health(&self) -> Result<serde_json::Value> {
        let url = format!("{base_url}/health", base_url = self.base_url);
        let req = self.client.get(&url);
        let response = Self::check_response(req.send().await?).await?;
        Ok(response.json().await?)
    }

    pub async fn get_dashboard(&self) -> Result<DashboardResponse> {
        let url = format!("{base_url}/rap/dashboard", base_url = self.base_url);
        let req = self.client.get(&url);
        let response = Self::check_response(req.send().await?).await?;
        Ok(response.json().await?)
    }

    pub async fn list_relationship_actions(&self, skip: Option<i64>, limit: Option<i64>, status: Option<String>, priority: Option<String>, contact_id: Option<String>) -> Result<serde_json::Value> {
        let url = format!("{base_url}/relationship-actions", base_url = self.base_url);
        let req = self.client.get(&url);
        let req = if let Some(v) = skip { req.query(&[("skip", v.to_string())]) } else { req };
        let req = if let Some(v) = limit { req.query(&[("limit", v.to_string())]) } else { req };
        let req = if let Some(v) = status { req.query(&[("status", v.to_string())]) } else { req };
        let req = if let Some(v) = priority { req.query(&[("priority", v.to_string())]) } else { req };
        let req = if let Some(v) = contact_id { req.query(&[("contact_id", v.to_string())]) } else { req };
        let response = Self::check_response(req.send().await?).await?;
        Ok(response.json().await?)
    }

    pub async fn list_due_relationship_actions(&self, skip: Option<i64>, limit: Option<i64>) -> Result<serde_json::Value> {
        let url = format!("{base_url}/relationship-actions/due", base_url = self.base_url);
        let req = self.client.get(&url);
        let req = if let Some(v) = skip { req.query(&[("skip", v.to_string())]) } else { req };
        let req = if let Some(v) = limit { req.query(&[("limit", v.to_string())]) } else { req };
        let response = Self::check_response(req.send().await?).await?;
        Ok(response.json().await?)
    }

    pub async fn update_relationship_action(&self, action_id: &str, body: &RelationshipActionUpdate) -> Result<RelationshipActionResponse> {
        let url = format!("{base_url}/relationship-actions/{action_id}", base_url = self.base_url, action_id = action_id);
        let req = self.client.patch(&url);
        let req = req.json(body);
        let response = Self::check_response(req.send().await?).await?;
        Ok(response.json().await?)
    }

    pub async fn complete_relationship_action(&self, action_id: &str) -> Result<RelationshipActionResponse> {
        let url = format!("{base_url}/relationship-actions/{action_id}/complete", base_url = self.base_url, action_id = action_id);
        let req = self.client.patch(&url);
        let response = Self::check_response(req.send().await?).await?;
        Ok(response.json().await?)
    }

}
