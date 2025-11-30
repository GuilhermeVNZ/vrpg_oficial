use thiserror::Error;

#[derive(Error, Debug)]
pub enum InfraError {
    #[error("Service error: {0}")]
    Service(String),

    #[error("Health check error: {0}")]
    HealthCheck(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, InfraError>;
