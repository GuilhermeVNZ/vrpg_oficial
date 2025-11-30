use crate::error::{MemoryError, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LexumQuery {
    pub query: String,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LexumResult {
    pub id: String,
    pub content: String,
    pub score: f32,
}

pub struct LexumClient {
    client: Client,
    endpoint: String,
}

impl Default for LexumClient {
    fn default() -> Self {
        Self::new("http://localhost:8001")
    }
}

impl LexumClient {
    pub fn new(endpoint: &str) -> Self {
        Self {
            client: Client::new(),
            endpoint: endpoint.to_string(),
        }
    }

    pub async fn search(&self, query: &str, limit: Option<usize>) -> Result<Vec<LexumResult>> {
        let request = LexumQuery {
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

        let results: Vec<LexumResult> = response
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
    async fn test_lexum_client_creation() {
        let client = LexumClient::new("http://localhost:8001");
        assert_eq!(client.endpoint, "http://localhost:8001");
    }
}
