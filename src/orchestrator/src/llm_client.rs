//! LLM Core Client
//!
//! Client for communicating with the LLM Core service
//! This will be used to generate INTENTs from player actions

use crate::error::{OrchestratorError, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// LLM Core Client
#[derive(Debug, Clone)]
pub struct LlmClient {
    client: Client,
    base_url: String,
}

impl Default for LlmClient {
    fn default() -> Self {
        Self::new("http://localhost:7002") // Default LLM Core service port
    }
}

impl LlmClient {
    /// Create a new LLM client
    pub fn new(base_url: &str) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(30)) // LLM inference can take time
                .build()
                .expect("Failed to create HTTP client"),
            base_url: base_url.to_string(),
        }
    }

    /// Generate response with INTENTs from LLM Core
    ///
    /// Sends a request to LLM Core with:
    /// - Player action text
    /// - Current scene state
    /// - Game context
    /// - Persona (DM, NPC, etc.)
    ///
    /// Returns narrative text and INTENT DSL blocks
    pub async fn generate_with_intents(&self, request: &LlmRequest) -> Result<LlmResponse> {
        let response = self
            .client
            .post(&format!("{}/llm", self.base_url))
            .json(request)
            .send()
            .await
            .map_err(|e| OrchestratorError::ServiceError(format!("LLM request failed: {}", e)))?;

        let status = response.status();
        let body = response.text().await.map_err(|e| {
            OrchestratorError::ServiceError(format!("LLM response read failed: {}", e))
        })?;

        if status.is_success() {
            serde_json::from_str(&body).map_err(|e| {
                OrchestratorError::ServiceError(format!("LLM deserialize failed: {}: {}", e, body))
            })
        } else {
            Err(OrchestratorError::ServiceError(format!(
                "LLM request failed with status {}: {}",
                status, body
            )))
        }
    }

    /// Check if LLM Core service is healthy
    pub async fn health_check(&self) -> Result<bool> {
        let response = self
            .client
            .get(&format!("{}/health", self.base_url))
            .send()
            .await
            .map_err(|e| {
                OrchestratorError::ServiceError(format!("LLM health check failed: {}", e))
            })?;

        Ok(response.status().is_success())
    }
}

/// LLM Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmRequest {
    /// Input text (player action, dialogue, etc.)
    pub text: String,
    /// Persona to use (dm, npc, player_ai, monster, narrator)
    pub persona: String,
    /// Current scene state
    pub scene_state: Option<String>,
    /// Game context (serialized game state)
    pub game_context: Option<serde_json::Value>,
    /// Memory context (relevant memories from memory service)
    pub memory_context: Option<Vec<String>>,
    /// Maximum tokens to generate
    pub max_tokens: Option<u32>,
    /// Temperature for generation
    pub temperature: Option<f32>,
}

/// LLM Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmResponse {
    /// Generated narrative text
    pub text: String,
    /// INTENT DSL blocks (if any)
    pub intents: Option<String>,
    /// Tokens generated
    pub tokens_generated: Option<u32>,
    /// Inference time in milliseconds
    pub inference_time_ms: Option<u64>,
}
