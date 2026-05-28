use thiserror::Error;

pub type Result<T> = std::result::Result<T, AkashaError>;

#[derive(Debug, Error)]
pub enum AkashaError {
    #[error("Storage error: {0}")]
    Storage(#[from] sled::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] bincode::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Record not found: {0}")]
    NotFound(String),

    #[error("Query parse error: {0}")]
    QueryParse(String),

    #[error("Query execution error: {0}")]
    QueryExecution(String),

    #[error("Embedding dimension mismatch: expected {expected}, got {got}")]
    DimensionMismatch { expected: usize, got: usize },

    #[error("Causal cycle detected: record {0} would create a cycle")]
    CausalCycle(String),

    #[error("Invalid record: {0}")]
    InvalidRecord(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Other: {0}")]
    Other(#[from] anyhow::Error),
}
