//! Game Session Management

pub mod persistence;

use crate::error::Result;
use crate::fsm::SceneStateMachine;
use chrono::{DateTime, Utc};
use game_engine::GameSession as EngineGameSession;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use persistence::{SessionPersistence, SerializableSession};

/// Game Session
///
/// Combines Orchestrator session management (FSM) with game-engine session (combat, scenes, actors)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameSession {
    pub session_id: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub state_machine: SceneStateMachine,
    /// Engine session for combat, scenes, and actors
    #[serde(skip)]
    pub engine_session: Option<EngineGameSession>,
}

impl GameSession {
    /// Create a new game session
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            session_id: Uuid::new_v4().to_string(),
            created_at: now,
            updated_at: now,
            state_machine: SceneStateMachine::new(),
            engine_session: Some(EngineGameSession::new("VRPG Session".to_string())),
        }
    }

    /// Create a new game session with a name
    pub fn with_name(name: String) -> Self {
        let now = Utc::now();
        Self {
            session_id: Uuid::new_v4().to_string(),
            created_at: now,
            updated_at: now,
            state_machine: SceneStateMachine::new(),
            engine_session: Some(EngineGameSession::new(name)),
        }
    }

    /// Get current scene state
    pub fn current_state(&self) -> crate::fsm::SceneState {
        self.state_machine.current_state()
    }

    /// Transition to new state
    pub fn transition_to(&mut self, new_state: crate::fsm::SceneState) -> Result<()> {
        self.state_machine.transition_to(new_state)?;
        self.updated_at = Utc::now();

        // Sync engine session state with FSM state
        if let Some(ref mut engine) = self.engine_session {
            match new_state {
                crate::fsm::SceneState::CombatTurnBased => {
                    // Ensure we have a current scene before starting combat
                    if engine.current_scene.is_none() {
                        engine.create_scene("Combat Scene".to_string());
                    }
                    // Combat will be started by INTENT executor
                }
                crate::fsm::SceneState::Exploration | crate::fsm::SceneState::SocialFreeFlow => {
                    // End combat if it was active
                    if let Some(scene) = engine.get_current_scene_mut() {
                        if scene.combat_active {
                            scene.end_combat();
                        }
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Get mutable reference to engine session
    pub fn engine_session_mut(&mut self) -> Option<&mut EngineGameSession> {
        self.engine_session.as_mut()
    }

    /// Get reference to engine session
    pub fn engine_session(&self) -> Option<&EngineGameSession> {
        self.engine_session.as_ref()
    }
}

impl Default for GameSession {
    fn default() -> Self {
        Self::new()
    }
}

/// Session Manager
pub struct SessionManager {
    sessions: std::collections::HashMap<String, GameSession>,
}

impl SessionManager {
    /// Create a new session manager
    pub fn new() -> Self {
        Self {
            sessions: std::collections::HashMap::new(),
        }
    }

    /// Create a new session
    pub fn create_session(&mut self) -> String {
        let session = GameSession::new();
        let session_id = session.session_id.clone();
        self.sessions.insert(session_id.clone(), session);
        session_id
    }

    /// Get a session
    pub fn get_session(&self, session_id: &str) -> Option<&GameSession> {
        self.sessions.get(session_id)
    }

    /// Get a mutable session
    pub fn get_session_mut(&mut self, session_id: &str) -> Option<&mut GameSession> {
        self.sessions.get_mut(session_id)
    }

    /// Remove a session
    pub fn remove_session(&mut self, session_id: &str) -> bool {
        self.sessions.remove(session_id).is_some()
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_session() {
        let session = GameSession::new();
        assert!(!session.session_id.is_empty());
    }

    #[test]
    fn test_session_manager() {
        let mut manager = SessionManager::new();
        let session_id = manager.create_session();
        assert!(manager.get_session(&session_id).is_some());
    }
}
