use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::error::{AsrError, Result};
use crate::vad::VoiceActivityDetector;
use crate::whisper::{AudioChunk, SharedWhisperModel, TranscriptionResult, WhisperModel};

#[derive(Clone)]
pub struct AppState {
    whisper_model: SharedWhisperModel,
    vad: Arc<VoiceActivityDetector>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
    pub model_loaded: bool,
}

#[derive(Debug, Deserialize)]
pub struct TranscribeChunkRequest {
    pub audio_data: Vec<f32>,
    pub sample_rate: u32,
    pub timestamp_ms: u64,
}

#[derive(Debug, Serialize)]
pub struct TranscribeChunkResponse {
    pub transcription: TranscriptionResult,
    pub vad_result: crate::vad::VadResult,
}

#[derive(Debug, Deserialize)]
pub struct TranscribeFinalRequest {
    pub audio_chunks: Vec<TranscribeChunkRequest>,
}

#[derive(Debug, Serialize)]
pub struct TranscribeFinalResponse {
    pub full_text: String,
    pub confidence: f32,
    pub language: String,
    pub duration_ms: u64,
}

pub struct AsrServer {
    state: AppState,
}

impl AsrServer {
    pub fn new() -> Result<Self> {
        let whisper_model = Arc::new(RwLock::new(WhisperModel::new()));
        let vad = Arc::new(VoiceActivityDetector::new());

        Ok(Self {
            state: AppState { whisper_model, vad },
        })
    }

    pub async fn load_model(&self, model_path: &str) -> Result<()> {
        let mut model = self.state.whisper_model.write().await;
        model.load(model_path).await
    }

    pub async fn start(&self, port: u16) -> Result<()> {
        let app = Router::new()
            .route("/health", get(health_handler))
            .route("/transcribe_chunk", post(transcribe_chunk_handler))
            .route("/transcribe_final", post(transcribe_final_handler))
            .layer(ServiceBuilder::new().layer(CorsLayer::permissive()))
            .with_state(self.state.clone());

        let addr = format!("0.0.0.0:{}", port);
        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .map_err(AsrError::Io)?;

        info!("ASR service listening on {}", addr);

        axum::serve(listener, app).await.map_err(AsrError::Io)?;

        Ok(())
    }
}

async fn health_handler(State(state): State<AppState>) -> Json<HealthResponse> {
    let model_loaded = state.whisper_model.read().await.is_loaded();
    Json(HealthResponse {
        status: "ok".to_string(),
        service: "asr-service".to_string(),
        version: "1.0.0".to_string(),
        model_loaded,
    })
}

async fn transcribe_chunk_handler(
    State(state): State<AppState>,
    Json(request): Json<TranscribeChunkRequest>,
) -> std::result::Result<Json<TranscribeChunkResponse>, (StatusCode, String)> {
    let chunk = AudioChunk {
        data: request.audio_data,
        sample_rate: request.sample_rate,
        timestamp_ms: request.timestamp_ms,
    };

    // Run VAD
    let vad_result = state.vad.detect(&chunk.data).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("VAD error: {}", e),
        )
    })?;

    // Transcribe with Whisper
    let model = state.whisper_model.read().await;
    let transcription = model.transcribe(&chunk).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Transcription error: {}", e),
        )
    })?;

    Ok(Json(TranscribeChunkResponse {
        transcription,
        vad_result,
    }))
}

async fn transcribe_final_handler(
    State(state): State<AppState>,
    Json(request): Json<TranscribeFinalRequest>,
) -> std::result::Result<Json<TranscribeFinalResponse>, (StatusCode, String)> {
    let model = state.whisper_model.read().await;

    let mut full_text_parts = Vec::new();
    let mut total_confidence = 0.0;
    let mut total_duration = 0u64;
    let mut language = "en".to_string();

    for chunk_req in &request.audio_chunks {
        let chunk = AudioChunk {
            data: chunk_req.audio_data.clone(),
            sample_rate: chunk_req.sample_rate,
            timestamp_ms: chunk_req.timestamp_ms,
        };

        let transcription = model.transcribe(&chunk).await.map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Transcription error: {}", e),
            )
        })?;

        full_text_parts.push(transcription.text);
        total_confidence += transcription.confidence;
        total_duration += transcription.duration_ms;
        language = transcription.language;
    }

    let full_text = full_text_parts.join(" ");
    let avg_confidence = if request.audio_chunks.is_empty() {
        0.0
    } else {
        total_confidence / request.audio_chunks.len() as f32
    };

    Ok(Json(TranscribeFinalResponse {
        full_text,
        confidence: avg_confidence,
        language,
        duration_ms: total_duration,
    }))
}
