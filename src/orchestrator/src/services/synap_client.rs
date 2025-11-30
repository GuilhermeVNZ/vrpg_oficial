//! Synap Client for Model Communication
//!
//! All AI model communications (ASR, TTS, LLM) go through Synap
//! This provides unified routing, queuing, and coordination

use crate::error::{OrchestratorError, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{error, info};

/// Synap Client for unified model communication
#[derive(Debug, Clone)]
pub struct SynapClient {
    client: Client,
    base_url: String,
    api_key: Option<String>,
}

impl SynapClient {
    /// Create a new Synap client
    pub fn new() -> Self {
        let base_url = std::env::var("SYNAP_ENDPOINT")
            .unwrap_or_else(|_| "http://localhost:15500".to_string());

        let api_key = std::env::var("SYNAP_API_KEY").ok();

        let client = Client::builder()
            .timeout(Duration::from_secs(60)) // Models can take time
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            base_url,
            api_key,
        }
    }

    /// Send a command to Synap using StreamableHTTP protocol
    async fn send_command<T: for<'de> Deserialize<'de>>(
        &self,
        command: &str,
        payload: serde_json::Value,
    ) -> Result<T> {
        // Generate unique request ID using timestamp + nanos
        let request_id = format!(
            "req-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );

        let request = json!({
            "command": command,
            "request_id": request_id,
            "payload": payload,
        });

        info!(
            "📡 Synap: Sending command '{}' (request_id: {})",
            command, request_id
        );

        let mut req = self
            .client
            .post(&format!("{}/api/v1/command", self.base_url))
            .json(&request);

        // Add API key if available
        if let Some(key) = &self.api_key {
            req = req.header("Authorization", format!("Bearer {}", key));
        }

        let response = req
            .send()
            .await
            .map_err(|e| OrchestratorError::ServiceError(format!("Synap request failed: {}", e)))?;

        let status = response.status();
        let body = response.text().await.map_err(|e| {
            OrchestratorError::ServiceError(format!("Failed to read Synap response: {}", e))
        })?;

        if !status.is_success() {
            error!("Synap returned error status {}: {}", status, body);
            return Err(OrchestratorError::ServiceError(format!(
                "Synap error {}: {}",
                status, body
            )));
        }

        let result: serde_json::Value = serde_json::from_str(&body).map_err(|e| {
            OrchestratorError::ServiceError(format!(
                "Failed to parse Synap response: {}: {}",
                e, body
            ))
        })?;

        // Check StreamableHTTP envelope
        if !result
            .get("success")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            let error_msg = result
                .get("error")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error");
            error!("Synap returned error: {}", error_msg);
            return Err(OrchestratorError::ServiceError(format!(
                "Synap error: {}",
                error_msg
            )));
        }

        // Extract payload
        let payload = result.get("payload").ok_or_else(|| {
            OrchestratorError::ServiceError("Missing payload in Synap response".to_string())
        })?;

        serde_json::from_value(payload.clone()).map_err(|e| {
            OrchestratorError::ServiceError(format!("Failed to deserialize Synap payload: {}", e))
        })
    }

    /// ASR: Transcribe audio via Synap
    pub async fn transcribe_audio(
        &self,
        audio_data: Vec<f32>,
        sample_rate: u32,
        language: Option<String>,
    ) -> Result<AsrResult> {
        let payload = json!({
            "operation": "asr.transcribe",
            "audio_data": audio_data,
            "sample_rate": sample_rate,
            "language": language.unwrap_or_else(|| "auto".to_string()),
            "model": "whisper-large-v3",
            "use_gpu": std::env::var("VRPG_ASR_USE_GPU")
                .unwrap_or_default()
                .to_lowercase() == "true",
        });

        self.send_command("asr.transcribe", payload).await
    }

    /// TTS: Synthesize speech via Synap
    pub async fn synthesize_speech(
        &self,
        text: &str,
        voice_id: &str,
        language: Option<String>,
        speed: Option<f32>,
        pitch: Option<f32>,
    ) -> Result<TtsResult> {
        let payload = json!({
            "operation": "tts.synthesize",
            "text": text,
            "voice_id": voice_id,
            "language": language.unwrap_or_else(|| "en".to_string()),
            "speed": speed.unwrap_or(1.0),
            "pitch": pitch.unwrap_or(0.0),
            "model": "xtts-v2",
            "use_gpu": std::env::var("VRPG_TTS_USE_GPU")
                .unwrap_or_default()
                .to_lowercase() == "true",
        });

        self.send_command("tts.synthesize", payload).await
    }

    /// LLM: Generate text via Synap
    pub async fn generate_text(
        &self,
        prompt: &str,
        persona: &str,
        max_tokens: Option<u32>,
        temperature: Option<f32>,
        context: Option<Vec<String>>,
    ) -> Result<LlmResult> {
        let payload = json!({
            "operation": "llm.generate",
            "model": std::env::var("VRPG_LLM_MODEL")
                .unwrap_or_else(|_| "qwen2.5-14b-instruct".to_string()),
            "prompt": prompt,
            "persona": persona,
            "max_tokens": max_tokens.unwrap_or(512),
            "temperature": temperature.unwrap_or(0.7),
            "context": context.unwrap_or_default(),
            "use_gpu": std::env::var("VRPG_LLM_USE_GPU")
                .unwrap_or_default()
                .to_lowercase() == "true",
        });

        self.send_command("llm.generate", payload).await
    }

    /// Health check
    pub async fn health_check(&self) -> Result<bool> {
        let response = self
            .client
            .get(&format!("{}/health", self.base_url))
            .send()
            .await
            .map_err(|e| {
                OrchestratorError::ServiceError(format!("Synap health check failed: {}", e))
            })?;

        Ok(response.status().is_success())
    }
}

impl Default for SynapClient {
    fn default() -> Self {
        Self::new()
    }
}

/// ASR Result from Synap
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AsrResult {
    pub text: String,
    pub confidence: f32,
    pub language: String,
    pub duration_ms: u64,
}

/// TTS Result from Synap
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsResult {
    pub audio: Vec<f32>,
    pub sample_rate: u32,
    pub channels: u16,
    pub duration_ms: u64,
}

/// LLM Result from Synap
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LlmResult {
    pub text: String,
    pub intents: Option<String>,
    pub tokens_used: u32,
    pub inference_time_ms: u64,
}

pub type SharedSynapClient = Arc<RwLock<SynapClient>>;
