//! Streaming Server
//!
//! WebSocket and SSE endpoints for real-time audio streaming to frontend

// Removed unused imports
use crate::streaming::{StreamingPipeline, StreamingRequest, StreamingStatus};
use axum::{
    extract::{ws::Message, State},
    response::{sse::Event, Sse},
    Json,
};
use futures::stream::{self, Stream};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{error, info};

/// Streaming request from frontend
#[derive(Debug, Deserialize)]
pub struct StreamingRequestPayload {
    pub text: String,
    pub character_id: String,
    pub language: String,
    /// Timestamp when user finished speaking (Unix timestamp in seconds)
    pub user_speech_end_ts: Option<f64>,
    /// LLM model name (for TTS profile selection)
    pub llm_model_name: Option<String>,
}

/// Streaming response to frontend
#[derive(Debug, Serialize)]
pub struct StreamingResponse {
    pub status: String,
    pub message: Option<String>,
    pub buffer_length: Option<f32>,
}

/// Audio chunk message for WebSocket
#[derive(Debug, Serialize)]
pub struct AudioChunkMessage {
    pub chunk_id: u64,
    pub samples: Vec<i16>,
    pub sample_rate: u32,
    pub channels: u16,
}

/// Handle WebSocket connection for audio streaming
pub async fn handle_websocket_stream(
    ws: axum::extract::ws::WebSocketUpgrade,
    State(pipeline): State<Arc<tokio::sync::RwLock<Option<StreamingPipeline>>>>,
) -> axum::response::Response {
    ws.on_upgrade(|socket| handle_websocket(socket, pipeline))
}

/// Handle WebSocket messages
async fn handle_websocket(
    mut socket: axum::extract::ws::WebSocket,
    pipeline: Arc<tokio::sync::RwLock<Option<StreamingPipeline>>>,
) {
    info!("WebSocket connection established for audio streaming");

    // Wait for streaming request
    while let Some(msg) = socket.recv().await {
        match msg {
            Ok(Message::Text(text)) => {
                // Parse request
                match serde_json::from_str::<StreamingRequestPayload>(&text) {
                    Ok(request) => {
                        info!("Received streaming request: {}", request.text);

                        // Get pipeline
                        let pipeline_guard = pipeline.read().await;
                        if let Some(pipeline) = pipeline_guard.as_ref() {
                            // Convert user_speech_end_ts from Unix timestamp to Instant
                            let user_speech_end_ts = request.user_speech_end_ts.map(|ts| {
                                let now = std::time::Instant::now();
                                let unix_now = std::time::SystemTime::now()
                                    .duration_since(std::time::UNIX_EPOCH)
                                    .unwrap()
                                    .as_secs_f64();
                                let diff_secs = unix_now - ts;
                                now.checked_sub(std::time::Duration::from_secs_f64(diff_secs))
                                    .unwrap_or(now)
                            });

                            // Start streaming
                            match pipeline.stream(StreamingRequest {
                                text: request.text,
                                character_id: request.character_id,
                                language: request.language,
                                profile: None,
                                llm_model_name: request.llm_model_name.clone(),
                                user_speech_end_ts,
                            }).await {
                                Ok(mut audio_rx) => {
                                    // Stream audio chunks
                                    let mut chunk_id = 0u64;
                                    while let Some(samples) = audio_rx.recv().await {
                                        chunk_id += 1;

                                        let message = AudioChunkMessage {
                                            chunk_id,
                                            samples,
                                            sample_rate: 24000,
                                            channels: 1,
                                        };

                                        let json = match serde_json::to_string(&message) {
                                            Ok(json) => json,
                                            Err(e) => {
                                                error!("Failed to serialize audio chunk: {}", e);
                                                continue;
                                            }
                                        };

                                        if socket.send(Message::Binary(json.into_bytes())).await.is_err() {
                                            error!("Failed to send audio chunk");
                                            break;
                                        }
                                    }

                                    // Send completion message
                                    let response = StreamingResponse {
                                        status: "completed".to_string(),
                                        message: Some("Streaming completed".to_string()),
                                        buffer_length: None,
                                    };
                                    if let Ok(json) = serde_json::to_string(&response) {
                                        let _ = socket.send(Message::Text(json)).await;
                                    }
                                }
                                Err(e) => {
                                    error!("Failed to start streaming: {}", e);
                                    let response = StreamingResponse {
                                        status: "error".to_string(),
                                        message: Some(format!("Failed to start streaming: {}", e)),
                                        buffer_length: None,
                                    };
                                    if let Ok(json) = serde_json::to_string(&response) {
                                        let _ = socket.send(Message::Text(json)).await;
                                    }
                                }
                            }
                        } else {
                            error!("Streaming pipeline not initialized");
                            let response = StreamingResponse {
                                status: "error".to_string(),
                                message: Some("Streaming pipeline not initialized".to_string()),
                                buffer_length: None,
                            };
                            if let Ok(json) = serde_json::to_string(&response) {
                                let _ = socket.send(Message::Text(json)).await;
                            }
                        }
                    }
                    Err(e) => {
                        error!("Failed to parse streaming request: {}", e);
                        let response = StreamingResponse {
                            status: "error".to_string(),
                            message: Some(format!("Invalid request: {}", e)),
                            buffer_length: None,
                        };
                        if let Ok(json) = serde_json::to_string(&response) {
                            let _ = socket.send(Message::Text(json)).await;
                        }
                    }
                }
            }
            Ok(Message::Close(_)) => {
                info!("WebSocket connection closed");
                break;
            }
            Ok(_) => {
                // Ignore other message types
            }
            Err(e) => {
                error!("WebSocket error: {}", e);
                break;
            }
        }
    }
}

/// Handle Server-Sent Events (SSE) for audio streaming
/// TODO: Fix type compatibility issues with SSE streams
#[allow(dead_code)]
pub async fn handle_sse_stream(
    State(_pipeline): State<Arc<tokio::sync::RwLock<Option<StreamingPipeline>>>>,
    Json(_request): Json<StreamingRequestPayload>,
) -> Sse<impl Stream<Item = std::result::Result<Event, Infallible>>> {
    // TODO: Fix SSE stream type compatibility
    // Temporarily return empty stream
    let empty_stream = stream::unfold((false, 0u64), |(sent, _chunk_id)| async move {
        if sent {
            None
        } else {
            Some((Ok(Event::default().data("error: SSE not yet implemented")), (true, 0)))
        }
    });
    Sse::new(empty_stream)
        .keep_alive(axum::response::sse::KeepAlive::new().interval(Duration::from_secs(15)))
}


/// Cancel streaming
pub async fn cancel_streaming(
    State(pipeline): State<Arc<tokio::sync::RwLock<Option<StreamingPipeline>>>>,
) -> Json<StreamingResponse> {
    let pipeline_guard = pipeline.read().await;
    if let Some(ref pipeline) = *pipeline_guard {
        if let Err(e) = pipeline.cancel().await {
            error!("Failed to cancel streaming: {}", e);
            return Json(StreamingResponse {
                status: "error".to_string(),
                message: Some(format!("Failed to cancel: {}", e)),
                buffer_length: None,
            });
        }

        Json(StreamingResponse {
            status: "cancelled".to_string(),
            message: Some("Streaming cancelled".to_string()),
            buffer_length: None,
        })
    } else {
        Json(StreamingResponse {
            status: "error".to_string(),
            message: Some("Streaming pipeline not initialized".to_string()),
            buffer_length: None,
        })
    }
}

/// Get streaming status
pub async fn get_streaming_status(
    State(pipeline): State<Arc<tokio::sync::RwLock<Option<StreamingPipeline>>>>,
) -> Json<StreamingResponse> {
    let pipeline_guard = pipeline.read().await;
    if let Some(pipeline) = pipeline_guard.as_ref() {
        let buffer_length = pipeline.buffer_length_seconds().unwrap_or(0.0);
        let state = pipeline.prebuffer_state().await;

        Json(StreamingResponse {
            status: format!("{:?}", state),
            message: None,
            buffer_length: Some(buffer_length),
        })
    } else {
        Json(StreamingResponse {
            status: "not_initialized".to_string(),
            message: Some("Streaming pipeline not initialized".to_string()),
            buffer_length: None,
        })
    }
}

