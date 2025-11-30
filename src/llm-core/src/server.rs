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

use crate::error::{LlmError, Result};
use crate::inference::{LlmInference, LlmRequest, LlmResponse};
use crate::persona::Persona;

#[derive(Clone)]
pub struct AppState {
    inference: Arc<RwLock<LlmInference>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
    pub model_loaded: bool,
    pub model_1_5b_loaded: bool,
    pub both_models_loaded: bool,
    pub current_persona: String,
}

#[derive(Debug, Deserialize)]
pub struct GenerateRequest {
    pub prompt: String,
    pub persona: String,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
    pub context: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct SetPersonaRequest {
    pub persona: String,
}

pub struct LlmServer {
    state: AppState,
}

impl LlmServer {
    pub fn new() -> Result<Self> {
        let inference = Arc::new(RwLock::new(LlmInference::new(Persona::DungeonMaster)));

        Ok(Self {
            state: AppState { inference },
        })
    }

    pub async fn load_model(&self, model_path: &str) -> Result<()> {
        let mut inference = self.state.inference.write().await;
        inference.load_model(model_path).await
    }

    pub async fn load_model_1_5b(&self, model_path: &str) -> Result<()> {
        let mut inference = self.state.inference.write().await;
        inference.load_model_1_5b(model_path).await
    }

    pub async fn start(&self, port: u16) -> Result<()> {
        let app = Router::new()
            .route("/health", get(health_handler))
            .route("/generate", post(generate_handler))
            .route("/llm/prelude", post(prelude_handler))
            .route("/set_persona", post(set_persona_handler))
            .route("/history", get(history_handler))
            .layer(ServiceBuilder::new().layer(CorsLayer::permissive()))
            .with_state(self.state.clone());

        let addr = format!("0.0.0.0:{}", port);
        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .map_err(LlmError::Io)?;

        info!("LLM Core service listening on {}", addr);

        axum::serve(listener, app).await.map_err(LlmError::Io)?;

        Ok(())
    }
}

async fn health_handler(State(state): State<AppState>) -> Json<HealthResponse> {
    let inference = state.inference.read().await;
    let model_loaded = inference.is_loaded();
    let model_1_5b_loaded = inference.is_1_5b_loaded();
    let both_models_loaded = inference.both_models_loaded();
    let current_persona = inference.persona().name().to_string();

    Json(HealthResponse {
        status: "ok".to_string(),
        service: "llm-core".to_string(),
        version: "1.0.0".to_string(),
        model_loaded,
        model_1_5b_loaded,
        both_models_loaded,
        current_persona,
    })
}

async fn generate_handler(
    State(state): State<AppState>,
    Json(request): Json<GenerateRequest>,
) -> std::result::Result<Json<LlmResponse>, (StatusCode, String)> {
    // Parse persona
    let persona = parse_persona(&request.persona)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid persona: {}", e)))?;

    let llm_request = LlmRequest {
        prompt: request.prompt,
        persona,
        max_tokens: request.max_tokens,
        temperature: request.temperature,
        context: request.context,
    };

    let inference = state.inference.read().await;
    let response = inference.generate(&llm_request).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Generation error: {}", e),
        )
    })?;

    Ok(Json(response))
}

async fn set_persona_handler(
    State(state): State<AppState>,
    Json(request): Json<SetPersonaRequest>,
) -> std::result::Result<Json<serde_json::Value>, (StatusCode, String)> {
    let persona = parse_persona(&request.persona)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid persona: {}", e)))?;

    {
        let mut inference = state.inference.write().await;
        inference.set_persona(persona);
    }

    let persona_name = {
        let inference = state.inference.read().await;
        inference.persona().name().to_string()
    };

    Ok(Json(serde_json::json!({
        "status": "ok",
        "persona": persona_name
    })))
}

async fn history_handler(State(state): State<AppState>) -> Json<Vec<(String, String)>> {
    let inference = state.inference.read().await;
    let history = inference.get_conversation_history().await;

    let history_json: Vec<(String, String)> = history
        .iter()
        .map(|(persona, text)| (persona.name().to_string(), text.clone()))
        .collect();

    Json(history_json)
}

/// Handler for /llm/prelude endpoint (1.5B fast inference)
async fn prelude_handler(
    State(state): State<AppState>,
    Json(request): Json<GenerateRequest>,
) -> std::result::Result<Json<LlmResponse>, (StatusCode, String)> {
    // Parse persona
    let persona = parse_persona(&request.persona)
        .map_err(|e| (StatusCode::BAD_REQUEST, format!("Invalid persona: {}", e)))?;

    let llm_request = LlmRequest {
        prompt: request.prompt,
        persona,
        max_tokens: Some(40),   // Hard limit for 1.5B
        temperature: Some(0.8), // Optimized for 1.5B
        context: request.context,
    };

    let inference = state.inference.read().await;

    // Check if 1.5B model is loaded
    if !inference.is_1_5b_loaded() {
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            "1.5B model not loaded. Call load_model_1_5b first.".to_string(),
        ));
    }

    let response = inference.infer_1_5b(&llm_request).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("1.5B inference error: {}", e),
        )
    })?;

    Ok(Json(response))
}

fn parse_persona(persona_str: &str) -> Result<Persona> {
    match persona_str.to_lowercase().as_str() {
        "dm" | "dungeon_master" | "dungeon master" => Ok(Persona::DungeonMaster),
        "narrator" => Ok(Persona::Narrator),
        _ => {
            // Try to parse as NPC, PlayerIA, or Monster
            // For now, default to NPC
            Ok(Persona::Npc(persona_str.to_string()))
        }
    }
}
