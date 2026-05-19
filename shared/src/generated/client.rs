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
        let _response = Self::check_response(req.send().await?).await?;
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

    pub async fn health(&self) -> Result<serde_json::Value> {
        let url = format!("{base_url}/health", base_url = self.base_url);
        let req = self.client.get(&url);
        let response = Self::check_response(req.send().await?).await?;
        Ok(response.json().await?)
    }

}
