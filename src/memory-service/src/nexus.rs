use crate::error::{MemoryError, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NexusQuery {
    pub query: String,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NexusResult {
    pub id: String,
    pub content: String,
    pub score: f32,
}

pub struct NexusClient {
    client: Client,
    endpoint: String,
}

impl Default for NexusClient {
    fn default() -> Self {
        Self::new("http://localhost:8000")
    }
}

impl NexusClient {
    pub fn new(endpoint: &str) -> Self {
        Self {
            client: Client::new(),
            endpoint: endpoint.to_string(),
        }
    }

    pub async fn search(&self, query: &str, limit: Option<usize>) -> Result<Vec<NexusResult>> {
        let request = NexusQuery {
            query: query.to_string(),
            limit,
        };

        let url = format!("{}/search", self.endpoint);
        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| MemoryError::Search(format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(MemoryError::Search(format!("HTTP {}: {}", status, text)));
        }

        let results: Vec<NexusResult> = response
            .json()
            .await
            .map_err(|e| MemoryError::Search(format!("Parse error: {}", e)))?;

        Ok(results)
    }

    pub async fn health_check(&self) -> Result<bool> {
        let url = format!("{}/health", self.endpoint);
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| MemoryError::Search(format!("Request failed: {}", e)))?;

        Ok(response.status().is_success())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_nexus_client_creation() {
        let client = NexusClient::new("http://localhost:8000");
        assert_eq!(client.endpoint, "http://localhost:8000");
    }
}
