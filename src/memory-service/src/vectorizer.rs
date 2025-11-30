use crate::error::{MemoryError, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorizeRequest {
    pub text: String,
    pub collection: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VectorizeResponse {
    pub vector_id: String,
    pub embedding: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchRequest {
    pub query: String,
    pub collection: Option<String>,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub text: String,
    pub score: f32,
}

pub struct VectorizerClient {
    client: Client,
    endpoint: String,
}

impl Default for VectorizerClient {
    fn default() -> Self {
        Self::new("http://localhost:8002")
    }
}

impl VectorizerClient {
    pub fn new(endpoint: &str) -> Self {
        Self {
            client: Client::new(),
            endpoint: endpoint.to_string(),
        }
    }

    pub async fn vectorize(
        &self,
        text: &str,
        collection: Option<&str>,
    ) -> Result<VectorizeResponse> {
        let request = VectorizeRequest {
            text: text.to_string(),
            collection: collection.map(|s| s.to_string()),
        };

        let url = format!("{}/vectorize", self.endpoint);
        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| MemoryError::Vectorizer(format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(MemoryError::Vectorizer(format!(
                "HTTP {}: {}",
                status, text
            )));
        }

        let result: VectorizeResponse = response
            .json()
            .await
            .map_err(|e| MemoryError::Vectorizer(format!("Parse error: {}", e)))?;

        Ok(result)
    }

    pub async fn search(
        &self,
        query: &str,
        collection: Option<&str>,
        limit: Option<usize>,
    ) -> Result<SearchResponse> {
        let request = SearchRequest {
            query: query.to_string(),
            collection: collection.map(|s| s.to_string()),
            limit,
        };

        let url = format!("{}/search", self.endpoint);
        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| MemoryError::Vectorizer(format!("Request failed: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(MemoryError::Vectorizer(format!(
                "HTTP {}: {}",
                status, text
            )));
        }

        let result: SearchResponse = response
            .json()
            .await
            .map_err(|e| MemoryError::Vectorizer(format!("Parse error: {}", e)))?;

        Ok(result)
    }

    pub async fn health_check(&self) -> Result<bool> {
        let url = format!("{}/health", self.endpoint);
        let response = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|e| MemoryError::Vectorizer(format!("Request failed: {}", e)))?;

        Ok(response.status().is_success())
    }
}

pub type SharedVectorizerClient = Arc<VectorizerClient>;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_vectorizer_client_creation() {
        let client = VectorizerClient::new("http://localhost:8002");
        assert_eq!(client.endpoint, "http://localhost:8002");
    }

    // Note: Integration tests would require a running Vectorizer service
}
