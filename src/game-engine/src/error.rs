use thiserror::Error;

#[derive(Error, Debug)]
pub enum GameError {
    #[error("Session error: {0}")]
    Session(String),

    #[error("State error: {0}")]
    State(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, GameError>;
