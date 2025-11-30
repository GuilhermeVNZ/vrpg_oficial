use thiserror::Error;

#[derive(Error, Debug)]
pub enum TtsError {
    #[error("Model loading failed: {0}")]
    ModelLoad(String),

    #[error("Synthesis failed: {0}")]
    Synthesis(String),

    #[error("Voice error: {0}")]
    Voice(String),

    #[error("Audio processing error: {0}")]
    Audio(String),

    #[error("ONNX Runtime error: {0}")]
    Onnx(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, TtsError>;
