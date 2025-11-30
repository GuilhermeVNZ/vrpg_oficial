use thiserror::Error;

#[derive(Error, Debug)]
pub enum RulesError {
    #[error("Dice parsing error: {0}")]
    DiceParse(String),

    #[error("Calculation error: {0}")]
    Calculation(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, RulesError>;
