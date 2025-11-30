use thiserror::Error;

#[derive(Error, Debug)]
pub enum MemoryError {
    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Search error: {0}")]
    Search(String),

    #[error("Vectorizer error: {0}")]
    Vectorizer(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, MemoryError>;
