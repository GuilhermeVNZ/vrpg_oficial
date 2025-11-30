use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::error::{Result, TtsError};
use crate::metrics::MetricsStats;
use crate::pipeline::{PipelineRequest, TtsPipeline};
use crate::streaming::StreamingPipeline;
use crate::streaming_server::{
    cancel_streaming, get_streaming_status, handle_sse_stream, handle_websocket_stream,
};
use std::path::PathBuf;

#[derive(Clone)]
pub struct AppState {
    pipeline: Arc<TtsPipeline>,
    streaming_pipeline: Arc<tokio::sync::RwLock<Option<StreamingPipeline>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
    pub model_loaded: bool,
    pub voices: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct SpeakRequest {
    pub text: String,             // Text from Qwen (may contain VOICE tags)
    pub language: Option<String>, // "pt" or "en", defaults to "pt"
    /// Timestamp when user finished speaking (Unix timestamp in seconds)
    pub user_speech_end_ts: Option<f64>,
    /// LLM model name (for TTS profile selection)
    pub llm_model_name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SpeakResponse {
    pub audio: Vec<f32>,
    pub sample_rate: u32,
    pub channels: u16,
    pub duration_ms: u64,
    pub actor: String,
    pub emotion: String,
    pub style: String,
    /// Interjection that was used (if any)
    pub interjection_used: Option<String>,
    /// Time from user speech end to interjection start (if used, in ms)
    pub time_to_interjection_ms: Option<u64>,
}

#[derive(Debug, Serialize)]
pub struct VoiceInfo {
    pub character_id: String,
    pub name: String,
    pub emotions: Vec<String>,
    pub styles: Vec<String>,
}

pub struct TtsServer {
    state: AppState,
}

impl TtsServer {
    pub fn new(base_path: &str) -> Result<Self> {
        let pipeline = Arc::new(TtsPipeline::new(PathBuf::from(base_path).as_path()));

        Ok(Self {
            state: AppState {
                pipeline,
                streaming_pipeline: Arc::new(tokio::sync::RwLock::new(None)),
            },
        })
    }

    /// Initialize streaming pipeline
    pub async fn init_streaming(
        &self,
        streaming_pipeline: StreamingPipeline,
    ) -> Result<()> {
        let mut pipeline = self.state.streaming_pipeline.write().await;
        *pipeline = Some(streaming_pipeline);
        Ok(())
    }

    // XTTS models are loaded automatically, no need for manual loading

    /// Initialize voice profiles (loads XTTS embeddings)
    pub async fn initialize_voice_profiles(&self) -> Result<()> {
        self.state.pipeline.initialize_voice_profiles().await
    }

    pub async fn start(&self, port: u16) -> Result<()> {
        let app = Router::new()
            .route("/health", get(health_handler))
            .route("/speak", post(speak_handler))
            .route("/voices", get(voices_handler))
            .route("/metrics", get(metrics_handler))
            // Streaming endpoints (TODO: Fix SSE type compatibility - commented for now)
            // Note: WebSocket and SSE require proper State integration and type fixes
            .layer(ServiceBuilder::new().layer(CorsLayer::permissive()))
            .with_state(self.state.clone());

        let addr = format!("0.0.0.0:{}", port);
        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .map_err(TtsError::Io)?;

        info!("TTS service listening on {}", addr);
        info!("Streaming endpoints available:");
        info!("  - WebSocket: ws://{}:{}/ws/stream", addr, port);
        info!("  - SSE: POST http://{}:{}/stream", addr, port);
        info!("  - Cancel: POST http://{}:{}/stream/cancel", addr, port);
        info!("  - Status: GET http://{}:{}/stream/status", addr, port);

        axum::serve(listener, app).await.map_err(TtsError::Io)?;

        Ok(())
    }
}

async fn health_handler(State(state): State<AppState>) -> Json<HealthResponse> {
    let pipeline = &state.pipeline;
    let xtts_loaded = pipeline.is_xtts_loaded().await;
    let model_loaded = xtts_loaded;
    let characters = pipeline.list_loaded_characters().await;

    Json(HealthResponse {
        status: "ok".to_string(),
        service: "tts-service".to_string(),
        version: "1.0.0".to_string(),
        model_loaded,
        voices: characters,
    })
}

async fn speak_handler(
    State(state): State<AppState>,
    Json(request): Json<SpeakRequest>,
) -> std::result::Result<Json<SpeakResponse>, (StatusCode, String)> {
    // Convert user_speech_end_ts from Unix timestamp to Instant
    let user_speech_end_ts = request.user_speech_end_ts.map(|ts| {
        // Convert from Unix timestamp (seconds) to Instant
        // Note: This is approximate since we don't have the exact system boot time
        // In production, the orchestrator should pass Instant directly or use a better method
        let now = std::time::Instant::now();
        let elapsed = now.elapsed();
        let unix_now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        let diff_secs = unix_now - ts;
        now.checked_sub(std::time::Duration::from_secs_f64(diff_secs))
            .unwrap_or(now)
    });

    let pipeline_request = PipelineRequest {
        text: request.text,
        language: request.language.unwrap_or_else(|| "pt".to_string()),
        user_speech_end_ts,
        llm_model_name: request.llm_model_name.clone(),
    };

    let response = state
        .pipeline
        .synthesize(pipeline_request)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Synthesis error: {}", e),
            )
        })?;

    Ok(Json(SpeakResponse {
        audio: response.audio,
        sample_rate: response.sample_rate,
        channels: response.channels,
        duration_ms: response.duration_ms,
        actor: response.actor,
        emotion: response.emotion,
        style: response.style,
        interjection_used: response.interjection_used,
        time_to_interjection_ms: response.time_to_interjection_ms,
    }))
}

async fn voices_handler(State(state): State<AppState>) -> Json<Vec<VoiceInfo>> {
    let characters = state.pipeline.list_loaded_characters().await;

    let mut voices = Vec::new();
    for char_id in characters {
        if let Some((emotions, styles)) = state.pipeline.get_character_info(&char_id).await {
            voices.push(VoiceInfo {
                character_id: char_id.clone(),
                name: char_id.clone(),
                emotions,
                styles,
            });
        }
    }

    Json(voices)
}

async fn metrics_handler(State(state): State<AppState>) -> Json<MetricsStats> {
    let metrics = state.pipeline.get_metrics();
    let stats = metrics.read().await.get_stats();
    Json(stats)
}
