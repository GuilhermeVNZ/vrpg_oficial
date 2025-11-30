use thiserror::Error;

#[derive(Error, Debug)]
pub enum AsrError {
    #[error("Model loading failed: {0}")]
    ModelLoad(String),

    #[error("Transcription failed: {0}")]
    Transcription(String),

    #[error("VAD error: {0}")]
    Vad(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, AsrError>;
