//! Communication layer for Orchestrator
//!
//! Handles IPC/WebSocket communication with Electron client

use crate::error::{OrchestratorError, Result};
use crate::orchestrator::Orchestrator;
use crate::session::SessionManager;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::Response,
    routing::get,
    Router,
};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use tracing::{error, info, warn};

/// Player Action from UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerAction {
    pub session_id: String,
    pub player_id: String,
    pub kind: ActionKind,
    pub text: Option<String>,
    pub ui_intent: Option<String>,
    pub target_id: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionKind {
    Voice,
    Ui,
}

impl std::fmt::Display for ActionKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionKind::Voice => write!(f, "Voice"),
            ActionKind::Ui => write!(f, "Ui"),
        }
    }
}

/// Roll Result from UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollResult {
    pub session_id: String,
    pub request_id: String,
    pub actor_id: String,
    pub total: i32,
    pub natural: i32,
    pub breakdown: serde_json::Value,
    pub client_seed: Option<String>,
    pub timestamp: i64,
}

/// Scene Update to UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneUpdate {
    pub session_id: String,
    pub scene_state: String,
    pub summary: String,
    pub active_speaker_id: Option<String>,
    pub participants: Vec<Participant>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    pub id: String,
    pub name: String,
    pub portrait_url: Option<String>,
    pub is_npc: bool,
}

/// Combat Update to UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CombatUpdate {
    pub session_id: String,
    pub in_combat: bool,
    pub round: u32,
    pub initiative_order: Vec<InitiativeEntry>,
    pub active_creature_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitiativeEntry {
    pub creature_id: String,
    pub name: String,
    pub current_hp: i32,
    pub max_hp: i32,
    pub is_active: bool,
}

/// Roll Request to UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollRequest {
    pub session_id: String,
    pub request_id: String,
    pub actor_id: String,
    pub roll_kind: String,
    pub skill: Option<String>,
    pub ability: Option<String>,
    pub dc: Option<i32>,
    pub formula_hint: Option<String>,
    pub reason: String,
}

/// Narration to UI/TTS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Narration {
    pub session_id: String,
    pub speaker_id: String,
    pub text: String,
    pub emotion: Option<String>,
    pub tagged_for_tts: bool,
}

/// IPC Message wrapper
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum IpcMessage {
    #[serde(rename = "player-action")]
    PlayerAction(PlayerAction),
    #[serde(rename = "roll-result")]
    RollResult(RollResult),
    #[serde(rename = "scene-update")]
    SceneUpdate(SceneUpdate),
    #[serde(rename = "combat-update")]
    CombatUpdate(CombatUpdate),
    #[serde(rename = "roll-request")]
    RollRequest(RollRequest),
    #[serde(rename = "narration")]
    Narration(Narration),
    #[serde(rename = "error")]
    Error(IpcError),
    #[serde(rename = "ping")]
    Ping,
    #[serde(rename = "pong")]
    Pong,
}

/// IPC Error response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IpcError {
    pub code: String,
    pub message: String,
    pub request_id: Option<String>,
}

/// Communication server state
#[derive(Clone)]
pub struct CommunicationState {
    session_manager: Arc<RwLock<SessionManager>>,
    /// Broadcast channel for sending messages to all connected clients
    tx: broadcast::Sender<IpcMessage>,
    /// Pending roll requests (request_id -> RollRequest)
    pending_rolls: Arc<RwLock<HashMap<String, RollRequest>>>,
    /// Orchestrator reference for processing actions
    orchestrator: Option<Arc<Orchestrator>>,
}

impl CommunicationState {
    pub fn new(session_manager: Arc<RwLock<SessionManager>>) -> Self {
        let (tx, _) = broadcast::channel(1000);
        Self {
            session_manager,
            tx,
            pending_rolls: Arc::new(RwLock::new(HashMap::new())),
            orchestrator: None,
        }
    }

    pub fn with_orchestrator(mut self, orchestrator: Arc<Orchestrator>) -> Self {
        self.orchestrator = Some(orchestrator);
        self
    }

    /// Send message to all connected clients
    pub fn broadcast(&self, message: IpcMessage) -> Result<()> {
        self.tx.send(message).map_err(|e| {
            OrchestratorError::CommunicationError(format!("Broadcast failed: {}", e))
        })?;
        Ok(())
    }

    /// Send message to specific session
    pub async fn send_to_session(&self, _session_id: &str, message: IpcMessage) -> Result<()> {
        // In a real implementation, we'd track which clients are connected to which sessions
        // For now, we broadcast to all and let clients filter
        self.broadcast(message)
    }

    /// Store pending roll request
    pub async fn store_roll_request(&self, request: RollRequest) {
        let mut pending = self.pending_rolls.write().await;
        pending.insert(request.request_id.clone(), request);
    }

    /// Get and remove pending roll request
    pub async fn get_roll_request(&self, request_id: &str) -> Option<RollRequest> {
        let mut pending = self.pending_rolls.write().await;
        pending.remove(request_id)
    }
}

/// Create WebSocket router
pub fn create_router(state: CommunicationState) -> Router {
    Router::new()
        .route("/ws", get(websocket_handler))
        .with_state(state)
}

/// WebSocket upgrade handler
async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<CommunicationState>,
) -> Response {
    ws.on_upgrade(|socket| handle_websocket(socket, state))
}

/// Handle WebSocket connection
async fn handle_websocket(socket: WebSocket, state: CommunicationState) {
    let (mut sender, mut receiver) = socket.split();
    let rx = state.tx.subscribe();

    info!("New WebSocket connection established");

    // Send welcome message
    let welcome = IpcMessage::Pong;
    if let Err(e) = sender
        .send(Message::Text(serde_json::to_string(&welcome).unwrap()))
        .await
    {
        error!("Failed to send welcome message: {}", e);
        return;
    }

    // Create a channel for ping/pong handling
    let (tx_pong, rx_pong) = tokio::sync::mpsc::channel::<Vec<u8>>(10);

    // Spawn task to handle outgoing messages (broadcasts)
    let mut send_task = {
        let mut sender_clone = sender;
        let mut rx_clone = rx;
        let mut rx_pong_clone = rx_pong;
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    // Handle broadcasts
                    Ok(msg) = rx_clone.recv() => {
                        let text = match serde_json::to_string(&msg) {
                            Ok(t) => t,
                            Err(e) => {
                                warn!("Failed to serialize message: {}", e);
                                continue;
                            }
                        };

                        if sender_clone.send(Message::Text(text.into())).await.is_err() {
                            break;
                        }
                    }
                    // Handle pong responses
                    Some(data) = rx_pong_clone.recv() => {
                        if sender_clone.send(Message::Pong(data)).await.is_err() {
                            break;
                        }
                    }
                }
            }
        })
    };

    // Handle incoming messages
    let mut recv_task = {
        let state_clone = state.clone();
        tokio::spawn(async move {
            while let Some(msg) = receiver.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        if let Err(e) = handle_incoming_message(&state_clone, &text).await {
                            error!("Error handling message: {}", e);
                            // Send error response
                            let error_msg = IpcMessage::Error(IpcError {
                                code: "processing_error".to_string(),
                                message: e.to_string(),
                                request_id: None,
                            });
                            let _ = state_clone.broadcast(error_msg);
                        }
                    }
                    Ok(Message::Close(_)) => {
                        info!("WebSocket connection closed");
                        break;
                    }
                    Ok(Message::Ping(data)) => {
                        if tx_pong.send(data).await.is_err() {
                            break;
                        }
                    }
                    Ok(_) => {}
                    Err(e) => {
                        warn!("WebSocket error: {}", e);
                        break;
                    }
                }
            }
        })
    };

    // Wait for either task to complete
    tokio::select! {
        _ = &mut send_task => {
            recv_task.abort();
        }
        _ = &mut recv_task => {
            send_task.abort();
        }
    }

    info!("WebSocket connection closed");
}

/// Handle incoming message from client
async fn handle_incoming_message(state: &CommunicationState, text: &str) -> Result<()> {
    let message: IpcMessage = serde_json::from_str(text).map_err(|e| {
        OrchestratorError::CommunicationError(format!("Failed to parse message: {}", e))
    })?;

    match message {
        IpcMessage::PlayerAction(action) => {
            info!(
                "Received PlayerAction: {} from {}",
                action.kind, action.player_id
            );
            // Process player action through orchestrator
            if let Some(ref orchestrator) = state.orchestrator {
                if let Err(e) = orchestrator.process_player_action(action).await {
                    error!("Failed to process PlayerAction: {}", e);
                    state.broadcast(IpcMessage::Error(crate::communication::IpcError {
                        code: "processing_error".to_string(),
                        message: e.to_string(),
                        request_id: None,
                    }))?;
                }
            } else {
                warn!("Orchestrator not available, cannot process PlayerAction");
            }
        }
        IpcMessage::RollResult(result) => {
            info!(
                "Received RollResult: request_id={}, total={}",
                result.request_id, result.total
            );

            // Process roll result through orchestrator
            if let Some(ref orchestrator) = state.orchestrator {
                if let Err(e) = orchestrator.process_roll_result(result).await {
                    error!("Failed to process RollResult: {}", e);
                    state.broadcast(IpcMessage::Error(crate::communication::IpcError {
                        code: "processing_error".to_string(),
                        message: e.to_string(),
                        request_id: None,
                    }))?;
                }
            } else {
                warn!("Orchestrator not available, cannot process RollResult");
            }
        }
        IpcMessage::Ping => {
            // Respond with pong
            state.broadcast(IpcMessage::Pong)?;
        }
        _ => {
            warn!("Unexpected message type from client");
        }
    }

    Ok(())
}

/// Start communication server
pub async fn start_server(state: CommunicationState, port: u16) -> Result<()> {
    let app = create_router(state);

    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .map_err(|e| OrchestratorError::CommunicationError(format!("Failed to bind: {}", e)))?;

    info!("Communication server listening on {}", addr);

    axum::serve(listener, app)
        .await
        .map_err(|e| OrchestratorError::CommunicationError(format!("Server error: {}", e)))?;

    Ok(())
}
