//! Session Persistence - M4.4
//! Complete session save/load system for continuity between sessions

use crate::cache::game_state_cache::GameStateCache;
use crate::cache::lore_cache::LoreCache;
use crate::cache::scene_context_cache::SceneContextCache;
use crate::error::{OrchestratorError, Result};
use crate::fsm::SceneState;
use crate::pipeline::{PipelineState, PipelineStatus};
use crate::session::GameSession;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, error, info, warn};

/// Session file format version
const SESSION_FORMAT_VERSION: u32 = 1;

/// Session metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionMetadata {
    pub format_version: u32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub saved_at: chrono::DateTime<chrono::Utc>,
    pub session_id: String,
    pub checksum: Option<String>,
    pub compressed: bool,
}

/// Complete session data for serialization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableSession {
    pub metadata: SessionMetadata,
    pub scene_state: SceneState,
    pub pipeline_state: SerializablePipelineState,
    pub game_state_cache: SerializableGameStateCache,
    pub scene_context_cache: SerializableSceneContextCache,
    pub lore_cache: SerializableLoreCache,
    pub action_history: Vec<String>,
    pub session_settings: HashMap<String, String>,
}

/// Serializable pipeline state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializablePipelineState {
    pub status: PipelineStatus,
    pub game_state: String,
    pub scene_context: String,
    pub lore_cache: String,
}

impl From<&PipelineState> for SerializablePipelineState {
    fn from(state: &PipelineState) -> Self {
        Self {
            status: *state.status(),
            game_state: state.game_state().to_string(),
            scene_context: state.scene_context().to_string(),
            lore_cache: state.lore_cache().to_string(),
        }
    }
}

/// Serializable game state cache (simplified - stores as JSON string)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableGameStateCache {
    pub data_json: String,
}

/// Serializable scene context cache (simplified - stores as JSON string)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableSceneContextCache {
    pub data_json: String,
}

/// Serializable lore cache (simplified - stores as JSON string)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableLoreCache {
    pub data_json: String,
}

/// Session persistence manager
pub struct SessionPersistence {
    save_directory: PathBuf,
}

impl SessionPersistence {
    /// Create new session persistence manager
    pub fn new<P: AsRef<Path>>(save_directory: P) -> Result<Self> {
        let save_dir = save_directory.as_ref().to_path_buf();
        
        // Create directory if it doesn't exist
        if !save_dir.exists() {
            fs::create_dir_all(&save_dir).map_err(|e| {
                OrchestratorError::ServiceError(format!(
                    "Failed to create save directory: {}",
                    e
                ))
            })?;
            info!("Created session save directory: {:?}", save_dir);
        }

        Ok(Self {
            save_directory: save_dir,
        })
    }

    /// Get save file path for session
    fn get_save_path(&self, session_id: &str) -> PathBuf {
        self.save_directory.join(format!("{}.json", session_id))
    }

    /// Save session to file
    pub fn save_session(
        &self,
        session: &GameSession,
        pipeline_state: &PipelineState,
        game_state_cache: &GameStateCache,
        scene_context_cache: &SceneContextCache,
        lore_cache: &LoreCache,
        action_history: Vec<String>,
        session_settings: HashMap<String, String>,
    ) -> Result<()> {
        let session_id = session.session_id.clone();
        let save_path = self.get_save_path(&session_id);

        info!("Saving session {} to {:?}", session_id, save_path);

        // Create serializable session
        let serializable_session = SerializableSession {
            metadata: SessionMetadata {
                format_version: SESSION_FORMAT_VERSION,
                created_at: session.created_at,
                saved_at: chrono::Utc::now(),
                session_id: session_id.clone(),
                checksum: None, // TODO: Calculate checksum
                compressed: false, // TODO: Add compression support
            },
            scene_state: session.current_state(),
            pipeline_state: SerializablePipelineState::from(pipeline_state),
            game_state_cache: SerializableGameStateCache {
                data_json: "{}".to_string(), // TODO: Serialize actual cache data
            },
            scene_context_cache: SerializableSceneContextCache {
                data_json: "{}".to_string(), // TODO: Serialize actual cache data
            },
            lore_cache: SerializableLoreCache {
                data_json: "{}".to_string(), // TODO: Serialize actual cache data
            },
            action_history,
            session_settings,
        };

        // Serialize to JSON
        let json = serde_json::to_string_pretty(&serializable_session).map_err(|e| {
            OrchestratorError::ServiceError(format!("Failed to serialize session: {}", e))
        })?;

        // Write to file
        fs::write(&save_path, json).map_err(|e| {
            OrchestratorError::ServiceError(format!(
                "Failed to write session file: {}",
                e
            ))
        })?;

        debug!("Session {} saved successfully", session_id);
        Ok(())
    }

    /// Load session from file
    pub fn load_session(
        &self,
        session_id: &str,
    ) -> Result<SerializableSession> {
        let save_path = self.get_save_path(session_id);

        if !save_path.exists() {
            return Err(OrchestratorError::ServiceError(format!(
                "Session file not found: {:?}",
                save_path
            )));
        }

        info!("Loading session {} from {:?}", session_id, save_path);

        // Read file
        let json = fs::read_to_string(&save_path).map_err(|e| {
            OrchestratorError::ServiceError(format!("Failed to read session file: {}", e))
        })?;

        // Deserialize
        let session: SerializableSession = serde_json::from_str(&json).map_err(|e| {
            OrchestratorError::ServiceError(format!("Failed to deserialize session: {}", e))
        })?;

        // Validate format version
        if session.metadata.format_version != SESSION_FORMAT_VERSION {
            warn!(
                "Session format version mismatch: expected {}, got {}",
                SESSION_FORMAT_VERSION, session.metadata.format_version
            );
            // TODO: Implement version migration
        }

        // Validate session ID
        if session.metadata.session_id != session_id {
            return Err(OrchestratorError::ServiceError(format!(
                "Session ID mismatch: expected {}, got {}",
                session_id, session.metadata.session_id
            )));
        }

        debug!("Session {} loaded successfully", session_id);
        Ok(session)
    }

    /// List all saved sessions
    pub fn list_sessions(&self) -> Result<Vec<String>> {
        let mut sessions = Vec::new();

        let entries = fs::read_dir(&self.save_directory).map_err(|e| {
            OrchestratorError::ServiceError(format!(
                "Failed to read save directory: {}",
                e
            ))
        })?;

        for entry in entries {
            let entry = entry.map_err(|e| {
                OrchestratorError::ServiceError(format!("Failed to read directory entry: {}", e))
            })?;

            let path = entry.path();
            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                    sessions.push(stem.to_string());
                }
            }
        }

        Ok(sessions)
    }

    /// Delete saved session
    pub fn delete_session(&self, session_id: &str) -> Result<()> {
        let save_path = self.get_save_path(session_id);

        if !save_path.exists() {
            return Err(OrchestratorError::ServiceError(format!(
                "Session file not found: {:?}",
                save_path
            )));
        }

        fs::remove_file(&save_path).map_err(|e| {
            OrchestratorError::ServiceError(format!("Failed to delete session file: {}", e))
        })?;

        info!("Session {} deleted successfully", session_id);
        Ok(())
    }

    /// Restore session state from serialized session
    pub fn restore_session_state(
        serializable: &SerializableSession,
    ) -> Result<(SceneState, PipelineState)> {
        // Create pipeline state from serialized data
        let mut pipeline_state = PipelineState::new();
        
        // Restore status (need to transition through valid states)
        // If transition fails (invalid state), keep default state
        pipeline_state.transition_to(serializable.pipeline_state.status).ok();
        
        // Restore state strings
        pipeline_state.update_game_state(serializable.pipeline_state.game_state.clone());
        pipeline_state.update_scene_context(serializable.pipeline_state.scene_context.clone());
        pipeline_state.update_lore_cache(serializable.pipeline_state.lore_cache.clone());

        Ok((serializable.scene_state, pipeline_state))
    }
}

impl Default for SessionPersistence {
    fn default() -> Self {
        // Default to "saves" directory in current directory
        Self::new("saves").unwrap_or_else(|e| {
            error!("Failed to create default save directory: {}", e);
            panic!("Cannot continue without save directory");
        })
    }
}

