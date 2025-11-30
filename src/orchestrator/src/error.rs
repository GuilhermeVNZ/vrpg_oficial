//! Error types for the Orchestrator

use thiserror::Error;

/// Result type for Orchestrator operations
pub type Result<T> = std::result::Result<T, OrchestratorError>;

/// Orchestrator error types
#[derive(Error, Debug)]
pub enum OrchestratorError {
    #[error("Invalid state transition: {0}")]
    InvalidStateTransition(String),

    #[error("INTENT parsing error: {0}")]
    IntentParseError(String),

    #[error("INTENT execution error: {0}")]
    IntentExecutionError(String),

    #[error("Session error: {0}")]
    SessionError(String),

    #[error("Communication error: {0}")]
    CommunicationError(String),

    #[error("Service integration error: {0}")]
    ServiceError(String),

    #[error("Game engine error: {0}")]
    GameEngineError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

impl From<game_engine::error::GameError> for OrchestratorError {
    fn from(err: game_engine::error::GameError) -> Self {
        OrchestratorError::GameEngineError(err.to_string())
    }
}
