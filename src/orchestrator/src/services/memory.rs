//! Integration with Memory Service
//!
//! This module provides HTTP client integration with the memory-service
//! for lore queries, rule lookups, and knowledge retrieval.

use crate::error::{OrchestratorError, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Memory Service client
pub struct MemoryClient {
    client: Client,
    base_url: String,
}

impl MemoryClient {
    /// Create a new Memory client
    pub fn new(base_url: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");

        Self { client, base_url }
    }

    /// Default client pointing to localhost:3002
    pub fn default() -> Self {
        Self::new("http://localhost:3002".to_string())
    }

    /// Search for memories (lore, rules, knowledge)
    pub async fn search(
        &self,
        query: &str,
        limit: Option<usize>,
        filters: Option<HashMap<String, String>>,
    ) -> Result<SearchResponse> {
        let request = SearchRequest {
            query: query.to_string(),
            limit,
            filters,
        };

        let response = self
            .client
            .post(&format!("{}/search", self.base_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                OrchestratorError::ServiceError(format!("Memory search request failed: {}", e))
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(OrchestratorError::ServiceError(format!(
                "Memory search failed with status {}: {}",
                status, text
            )));
        }

        let results: Vec<SearchResult> = response.json().await.map_err(|e| {
            OrchestratorError::ServiceError(format!("Failed to parse search response: {}", e))
        })?;

        Ok(SearchResponse { results })
    }

    /// Store a new memory
    pub async fn store(
        &self,
        content: &str,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<StoreResponse> {
        let request = StoreRequest {
            content: content.to_string(),
            metadata,
        };

        let response = self
            .client
            .post(&format!("{}/store", self.base_url))
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                OrchestratorError::ServiceError(format!("Memory store request failed: {}", e))
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(OrchestratorError::ServiceError(format!(
                "Memory store failed with status {}: {}",
                status, text
            )));
        }

        let result: StoreResponse = response.json().await.map_err(|e| {
            OrchestratorError::ServiceError(format!("Failed to parse store response: {}", e))
        })?;

        Ok(result)
    }

    /// Get a specific memory by ID
    pub async fn get(&self, id: &str) -> Result<MemoryResponse> {
        let response = self
            .client
            .get(&format!("{}/get/{}", self.base_url, id))
            .send()
            .await
            .map_err(|e| {
                OrchestratorError::ServiceError(format!("Memory get request failed: {}", e))
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(OrchestratorError::ServiceError(format!(
                "Memory get failed with status {}: {}",
                status, text
            )));
        }

        let result: MemoryResponse = response.json().await.map_err(|e| {
            OrchestratorError::ServiceError(format!("Failed to parse memory response: {}", e))
        })?;

        Ok(result)
    }
}

// Request/Response types matching memory-service API

#[derive(Debug, Serialize, Deserialize)]
struct SearchRequest {
    query: String,
    limit: Option<usize>,
    filters: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResult>,
}

impl SearchResponse {
    pub fn new(results: Vec<SearchResult>) -> Self {
        Self { results }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub content: String,
    pub metadata: HashMap<String, String>,
    pub score: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct StoreRequest {
    content: String,
    metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreResponse {
    pub id: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryResponse {
    pub id: String,
    pub content: String,
    pub metadata: HashMap<String, String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}
