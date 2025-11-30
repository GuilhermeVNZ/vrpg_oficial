//! Pipeline State Management - M2.1
//! Manages pipeline state for the 3-agent pipeline (Orchestrator + Qwen-1.5B + Qwen-14B)

pub mod context_14b;
pub mod flow;
pub mod objective_responses;
pub mod simple_rule_query;
pub mod trigger;

use crate::error::{OrchestratorError, Result};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

/// Pipeline status enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PipelineStatus {
    /// Waiting for user input
    WaitingForInput,
    /// Processing with 1.5B (fast prelude)
    Processing1_5B,
    /// Waiting for final ASR transcription
    WaitingForFinalASR,
    /// Processing with 14B (full narrative)
    Processing14B,
    /// Ready for TTS output
    ReadyForTTS,
}

impl PipelineStatus {
    /// Get human-readable name
    pub fn name(&self) -> &'static str {
        match self {
            PipelineStatus::WaitingForInput => "WaitingForInput",
            PipelineStatus::Processing1_5B => "Processing1_5B",
            PipelineStatus::WaitingForFinalASR => "WaitingForFinalASR",
            PipelineStatus::Processing14B => "Processing14B",
            PipelineStatus::ReadyForTTS => "ReadyForTTS",
        }
    }

    /// Check if transition from `from` to `to` is valid
    pub fn can_transition_from(from: PipelineStatus, to: PipelineStatus) -> bool {
        match (from, to) {
            // From WaitingForInput
            (PipelineStatus::WaitingForInput, PipelineStatus::Processing1_5B) => true,

            // From Processing1_5B
            (PipelineStatus::Processing1_5B, PipelineStatus::WaitingForFinalASR) => true,

            // From WaitingForFinalASR
            (PipelineStatus::WaitingForFinalASR, PipelineStatus::Processing14B) => true,

            // From Processing14B
            (PipelineStatus::Processing14B, PipelineStatus::ReadyForTTS) => true,

            // From ReadyForTTS
            (PipelineStatus::ReadyForTTS, PipelineStatus::WaitingForInput) => true,

            // All other transitions are invalid
            _ => false,
        }
    }
}

/// Pipeline state structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineState {
    /// Current pipeline status
    status: PipelineStatus,
    /// Game state (RAM)
    game_state: String,
    /// Scene context (RAM + Vector)
    scene_context: String,
    /// Lore cache (Vectorizer)
    lore_cache: String,
}

impl PipelineState {
    /// Create new pipeline state
    pub fn new() -> Self {
        Self {
            status: PipelineStatus::WaitingForInput,
            game_state: String::new(),
            scene_context: String::new(),
            lore_cache: String::new(),
        }
    }

    /// Get current status
    pub fn status(&self) -> &PipelineStatus {
        &self.status
    }

    /// Get game state
    pub fn game_state(&self) -> &str {
        &self.game_state
    }

    /// Get scene context
    pub fn scene_context(&self) -> &str {
        &self.scene_context
    }

    /// Get lore cache
    pub fn lore_cache(&self) -> &str {
        &self.lore_cache
    }

    /// Transition to new status
    pub fn transition_to(&mut self, new_status: PipelineStatus) -> Result<()> {
        if !PipelineStatus::can_transition_from(self.status, new_status) {
            return Err(OrchestratorError::InvalidStateTransition(format!(
                "from {} to {}",
                self.status.name(),
                new_status.name()
            )));
        }

        self.status = new_status;
        Ok(())
    }

    /// Update game state
    pub fn update_game_state(&mut self, state: String) {
        self.game_state = state;
    }

    /// Update scene context
    pub fn update_scene_context(&mut self, context: String) {
        self.scene_context = context;
    }

    /// Update lore cache
    pub fn update_lore_cache(&mut self, lore: String) {
        self.lore_cache = lore;
    }
}

impl Default for PipelineState {
    fn default() -> Self {
        Self::new()
    }
}

/// Thread-safe pipeline state manager
pub struct PipelineStateManager {
    state: Arc<Mutex<PipelineState>>,
}

impl PipelineStateManager {
    /// Create new pipeline state manager
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(PipelineState::new())),
        }
    }

    /// Get state (thread-safe)
    pub fn get_state(&self) -> Result<PipelineState> {
        let state = self.state.lock().map_err(|_| {
            OrchestratorError::ServiceError("Failed to acquire pipeline state lock".to_string())
        })?;
        Ok(state.clone())
    }

    /// Transition to new status (thread-safe)
    pub fn transition_to(&self, new_status: PipelineStatus) -> Result<()> {
        let mut state = self.state.lock().map_err(|_| {
            OrchestratorError::ServiceError("Failed to acquire pipeline state lock".to_string())
        })?;
        state.transition_to(new_status)
    }
}

impl Default for PipelineStateManager {
    fn default() -> Self {
        Self::new()
    }
}
