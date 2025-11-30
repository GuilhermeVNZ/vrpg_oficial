//! Main Orchestrator module
//!
//! Coordinates the complete flow:
//! 1. Receives PlayerAction (voice or UI)
//! 2. Sends to LLM Core for INTENT generation
//! 3. Parses and executes INTENTs
//! 4. Sends updates back to client

use crate::communication::{CommunicationState, IpcMessage, PlayerAction, RollResult};
use crate::error::{OrchestratorError, Result};
use crate::intent::{IntentExecutor, IntentParser};
use crate::llm_client::{LlmClient, LlmRequest};
use crate::services::{SharedTtsClient, TtsClient};
use crate::session::{GameSession, SessionManager};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{error, info, warn};

/// Main Orchestrator
pub struct Orchestrator {
    /// Session manager for game sessions
    session_manager: Arc<RwLock<SessionManager>>,
    /// INTENT executor for processing INTENTs
    intent_executor: Arc<IntentExecutor>,
    /// Communication state for IPC/WebSocket
    communication: Arc<CommunicationState>,
    /// LLM Core client (for generating INTENTs from player actions)
    llm_client: Option<Arc<LlmClient>>,
    /// TTS Service client (for voice synthesis)
    tts_client: Option<SharedTtsClient>,
}

impl Orchestrator {
    /// Create a new Orchestrator
    pub fn new(
        session_manager: Arc<RwLock<SessionManager>>,
        communication: Arc<CommunicationState>,
    ) -> Self {
        Self {
            session_manager,
            intent_executor: Arc::new(IntentExecutor::new()),
            communication,
            llm_client: None, // Will be set when LLM Core is available
            tts_client: Some(Arc::new(TtsClient::new())), // TTS client available by default
        }
    }

    /// Create with LLM client
    pub fn with_llm_client(
        session_manager: Arc<RwLock<SessionManager>>,
        communication: Arc<CommunicationState>,
        llm_client: Arc<LlmClient>,
    ) -> Self {
        Self {
            session_manager,
            intent_executor: Arc::new(IntentExecutor::new()),
            communication,
            llm_client: Some(llm_client),
            tts_client: Some(Arc::new(TtsClient::new())),
        }
    }

    /// Set TTS client (for custom configuration)
    pub fn set_tts_client(&mut self, tts_client: SharedTtsClient) {
        self.tts_client = Some(tts_client);
    }

    /// Set LLM client (for lazy initialization)
    pub fn set_llm_client(&mut self, llm_client: Arc<LlmClient>) {
        self.llm_client = Some(llm_client);
    }

    /// Process a PlayerAction
    ///
    /// Flow:
    /// 1. Get or create session
    /// 2. Send action to LLM Core (if voice/text)
    /// 3. Parse INTENTs from LLM response
    /// 4. Execute INTENTs
    /// 5. Send updates to client
    pub async fn process_player_action(&self, action: PlayerAction) -> Result<()> {
        info!(
            "Processing PlayerAction: {} from {}",
            action.kind, action.player_id
        );

        // Get or create session
        let session_id = action.session_id.clone();
        let mut session_manager = self.session_manager.write().await;

        let session = session_manager
            .get_session_mut(&session_id)
            .ok_or_else(|| {
                OrchestratorError::SessionError(format!("Session not found: {}", session_id))
            })?;

        // Process action based on kind
        match action.kind {
            crate::communication::ActionKind::Voice => {
                // Voice action: send to LLM Core for INTENT generation
                if let Some(text) = &action.text {
                    self.process_voice_action(session, text, &action).await?;
                } else {
                    warn!("Voice action without text");
                }
            }
            crate::communication::ActionKind::Ui => {
                // UI action: process directly (e.g., button clicks, menu selections)
                self.process_ui_action(session, &action).await?;
            }
        }

        // Send scene update to client
        self.send_scene_update(&session_id, session).await?;

        Ok(())
    }

    /// Process voice action (sends to LLM Core)
    async fn process_voice_action(
        &self,
        session: &mut GameSession,
        text: &str,
        action: &PlayerAction,
    ) -> Result<()> {
        info!("Processing voice action: {}", text);

        // Send to LLM Core for INTENT generation (if available)
        let intent_text = if let Some(ref llm_client) = self.llm_client {
            // Check if LLM Core is available
            match llm_client.health_check().await {
                Ok(true) => {
                    // LLM Core is available, use it
                    let llm_request = LlmRequest {
                        text: text.to_string(),
                        persona: "dm".to_string(), // DM persona for player actions
                        scene_state: Some(session.current_state().name().to_string()),
                        game_context: Some(serde_json::json!({
                            "context": self.serialize_game_context(session)
                        })),
                        memory_context: self.get_memory_context(&action.session_id).await,
                        max_tokens: Some(2048),
                        temperature: Some(0.7),
                    };

                    match llm_client.generate_with_intents(&llm_request).await {
                        Ok(llm_response) => {
                            // Combine narrative and INTENTs
                            let mut combined = llm_response.text.clone();
                            if let Some(intents) = &llm_response.intents {
                                combined.push_str("\n\n");
                                combined.push_str(intents);
                            }
                            combined
                        }
                        Err(e) => {
                            warn!("LLM Core request failed: {}, using fallback", e);
                            // Fallback to simple INTENT
                            self.create_fallback_intent(&action.player_id, text)
                        }
                    }
                }
                Ok(false) | Err(_) => {
                    warn!("LLM Core not available, using fallback");
                    self.create_fallback_intent(&action.player_id, text)
                }
            }
        } else {
            // No LLM client configured, use fallback
            self.create_fallback_intent(&action.player_id, text)
        };

        // Parse INTENTs from LLM response
        let intents = IntentParser::parse(&intent_text)?;

        // Extract narrative text (everything outside INTENT blocks)
        let narrative = self.extract_narrative(&intent_text);

        // Send narration to client/TTS
        if !narrative.trim().is_empty() {
            let narrative_text = narrative.clone();

            // Send text to client
            self.communication.broadcast(IpcMessage::Narration(
                crate::communication::Narration {
                    session_id: session.session_id.clone(),
                    speaker_id: "dm".to_string(),
                    text: narrative_text.clone(),
                    emotion: None,
                    tagged_for_tts: true,
                },
            ))?;

            // Synthesize speech if TTS client is available
            if let Some(tts_client) = &self.tts_client {
                match tts_client.speak(&narrative_text, Some("pt")).await {
                    Ok(tts_response) => {
                        info!(
                            "TTS synthesis successful: {}ms audio, actor={}, emotion={}",
                            tts_response.duration_ms, tts_response.actor, tts_response.emotion
                        );
                        // Audio is ready - can be sent to client via IPC if needed
                        // For now, the client can request audio separately if needed
                    }
                    Err(e) => {
                        warn!("TTS synthesis failed: {}, continuing without audio", e);
                        // Continue without audio - not critical for gameplay
                    }
                }
            }
        }

        // Execute all INTENTs
        for intent in &intents {
            if let Err(e) = self.intent_executor.execute(intent, session).await {
                error!("Failed to execute INTENT: {}", e);
                // Continue with other INTENTs even if one fails
            }
        }

        Ok(())
    }

    /// Process UI action (direct action, no LLM needed)
    async fn process_ui_action(
        &self,
        session: &mut GameSession,
        action: &PlayerAction,
    ) -> Result<()> {
        info!("Processing UI action: {:?}", action.ui_intent);

        // UI actions are direct commands (e.g., "end_turn", "use_item", "move_token")
        // These don't need LLM processing, they're executed directly

        if let Some(ui_intent) = &action.ui_intent {
            match ui_intent.as_str() {
                "end_turn" => {
                    // End current turn in combat
                    if let Some(engine) = session.engine_session_mut() {
                        engine.next_turn()?;
                    }
                }
                "use_item" => {
                    // Use item from inventory
                    // TODO: Implement item usage
                }
                "move_token" => {
                    // Move token on map
                    // TODO: Implement token movement
                }
                _ => {
                    warn!("Unknown UI intent: {}", ui_intent);
                }
            }
        }

        Ok(())
    }

    /// Process RollResult from client
    pub async fn process_roll_result(&self, result: RollResult) -> Result<()> {
        info!(
            "Processing RollResult: request_id={}, total={}",
            result.request_id, result.total
        );

        // Get pending roll request
        if let Some(request) = self
            .communication
            .get_roll_request(&result.request_id)
            .await
        {
            info!(
                "Matched RollResult to RollRequest for actor {}",
                request.actor_id
            );

            // Get session
            let mut session_manager = self.session_manager.write().await;
            let session_id = result.session_id.clone();

            if let Some(_session) = session_manager.get_session_mut(&session_id) {
                // Process roll result based on roll_kind
                match request.roll_kind.as_str() {
                    "skill_check" => {
                        // Skill check completed, continue with narrative
                        // TODO: Send result to LLM for narrative continuation
                    }
                    "attack" => {
                        // Attack roll completed, calculate damage if hit
                        // TODO: Process attack result
                    }
                    _ => {
                        warn!("Unknown roll kind: {}", request.roll_kind);
                    }
                }
            }
        } else {
            warn!(
                "Orphaned RollResult: request_id={} not found",
                result.request_id
            );
            return Err(OrchestratorError::CommunicationError(format!(
                "RollResult request_id {} not found",
                result.request_id
            )));
        }

        Ok(())
    }

    /// Send scene update to client
    async fn send_scene_update(&self, session_id: &str, session: &GameSession) -> Result<()> {
        let scene_update = IpcMessage::SceneUpdate(crate::communication::SceneUpdate {
            session_id: session_id.to_string(),
            scene_state: session.current_state().name().to_string(),
            summary: format!("Scene in {:?} state", session.current_state()),
            active_speaker_id: None,
            participants: vec![],
        });

        self.communication.broadcast(scene_update)?;
        Ok(())
    }

    /// Extract narrative text from LLM response (removes INTENT blocks)
    fn extract_narrative(&self, text: &str) -> String {
        // Remove INTENT blocks to get pure narrative
        let start_marker = "[INTENTS]";
        let end_marker = "[/INTENTS]";

        let mut remaining = text;
        let mut result = String::new();

        while let Some(start_idx) = remaining.find(start_marker) {
            // Add text before INTENT block
            result.push_str(&remaining[..start_idx]);

            // Find end of INTENT block
            let block_start = start_idx + start_marker.len();
            let block_text = &remaining[block_start..];

            if let Some(end_idx) = block_text.find(end_marker) {
                // Move past this block
                remaining = &block_text[end_idx + end_marker.len()..];
            } else {
                // No closing marker, break
                break;
            }
        }

        // Add remaining text
        result.push_str(remaining);

        result.trim().to_string()
    }

    /// Create a fallback INTENT when LLM Core is not available
    fn create_fallback_intent(&self, player_id: &str, text: &str) -> String {
        // Simple fallback: create a skill check INTENT
        format!(
            r#"
[INTENTS]
INTENT: SKILL_CHECK
ACTOR: {}
SKILL: perception
CONTEXT: "{}"
SUGGEST_DC: YES
END_INTENT
[/INTENTS]
"#,
            player_id, text
        )
    }

    /// Serialize game context for LLM
    fn serialize_game_context(&self, session: &GameSession) -> String {
        // Serialize current game state for LLM context
        let state = session.current_state();
        let mut context = format!("State: {:?}\n", state);

        // Add scene information if available
        if let Some(engine) = session.engine_session() {
            if let Some(scene) = engine.get_current_scene() {
                context.push_str(&format!("Scene: {}\n", scene.name));
                context.push_str(&format!("Actors: {}\n", scene.all_actors().len()));
            }
        }

        context
    }

    /// Get relevant memory context for LLM
    async fn get_memory_context(&self, _session_id: &str) -> Option<Vec<String>> {
        // TODO: Query memory service for relevant context
        // For now, return None
        None
    }
}
