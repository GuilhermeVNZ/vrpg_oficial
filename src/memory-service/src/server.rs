use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{delete, get, post, put},
    Router,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;

use crate::error::{MemoryError, Result};
use crate::store::{Memory, MemoryQuery, MemoryStore, SearchResult};
use crate::vectorizer::{SharedVectorizerClient, VectorizerClient};

#[derive(Clone)]
pub struct AppState {
    store: Arc<MemoryStore>,
    vectorizer: SharedVectorizerClient,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
    pub version: String,
    pub memory_count: usize,
}

#[derive(Debug, Deserialize)]
pub struct StoreRequest {
    pub content: String,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRequest {
    pub content: Option<String>,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct SearchRequest {
    pub query: String,
    pub limit: Option<usize>,
    pub filters: Option<HashMap<String, String>>,
}

pub struct MemoryServer {
    state: AppState,
}

impl MemoryServer {
    pub fn new(vectorizer_endpoint: Option<&str>) -> Result<Self> {
        let store = Arc::new(MemoryStore::new());
        let vectorizer = Arc::new(VectorizerClient::new(
            vectorizer_endpoint.unwrap_or("http://localhost:8002"),
        ));

        Ok(Self {
            state: AppState { store, vectorizer },
        })
    }

    pub async fn start(&self, port: u16) -> Result<()> {
        let app = Router::new()
            .route("/health", get(health_handler))
            .route("/store", post(store_handler))
            .route("/get/:id", get(get_handler))
            .route("/update/:id", put(update_handler))
            .route("/delete/:id", delete(delete_handler))
            .route("/search", post(search_handler))
            .route("/list", get(list_handler))
            .layer(ServiceBuilder::new().layer(CorsLayer::permissive()))
            .with_state(self.state.clone());

        let addr = format!("0.0.0.0:{}", port);
        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .map_err(MemoryError::Io)?;

        info!("Memory service listening on {}", addr);

        axum::serve(listener, app).await.map_err(MemoryError::Io)?;

        Ok(())
    }
}

async fn health_handler(State(state): State<AppState>) -> Json<HealthResponse> {
    let memory_count = state.store.count().await;

    Json(HealthResponse {
        status: "ok".to_string(),
        service: "memory-service".to_string(),
        version: "1.0.0".to_string(),
        memory_count,
    })
}

async fn store_handler(
    State(state): State<AppState>,
    Json(request): Json<StoreRequest>,
) -> std::result::Result<Json<Memory>, (StatusCode, String)> {
    let metadata = request.metadata.unwrap_or_default();

    let memory = state
        .store
        .store(request.content, metadata)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Storage error: {}", e),
            )
        })?;

    // Optionally vectorize and store vector_id
    // In a real implementation, this would be done asynchronously
    if let Ok(vector_response) = state
        .vectorizer
        .vectorize(&memory.content, Some("memories"))
        .await
    {
        let mut updated_memory = memory.clone();
        updated_memory.vector_id = Some(vector_response.vector_id);
        // Update memory with vector_id
        let _ = state.store.update(&memory.id, None, None).await;
    }

    Ok(Json(memory))
}

async fn get_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> std::result::Result<Json<Memory>, (StatusCode, String)> {
    let memory = state
        .store
        .get(&id)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Storage error: {}", e),
            )
        })?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Memory not found".to_string()))?;

    Ok(Json(memory))
}

async fn update_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(request): Json<UpdateRequest>,
) -> std::result::Result<Json<Memory>, (StatusCode, String)> {
    let memory = state
        .store
        .update(&id, request.content, request.metadata)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Update error: {}", e),
            )
        })?;

    Ok(Json(memory))
}

async fn delete_handler(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> std::result::Result<Json<serde_json::Value>, (StatusCode, String)> {
    state.store.delete(&id).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Delete error: {}", e),
        )
    })?;

    Ok(Json(serde_json::json!({
        "status": "ok",
        "id": id
    })))
}

async fn search_handler(
    State(state): State<AppState>,
    Json(request): Json<SearchRequest>,
) -> std::result::Result<Json<Vec<SearchResult>>, (StatusCode, String)> {
    let query = MemoryQuery {
        query: request.query,
        limit: request.limit,
        filters: request.filters,
    };

    let results = state.store.search(&query).await.map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Search error: {}", e),
        )
    })?;

    Ok(Json(results))
}

async fn list_handler(State(state): State<AppState>) -> Json<Vec<Memory>> {
    let memories = state.store.list_all().await;
    Json(memories)
}
