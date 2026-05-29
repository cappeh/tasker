
#[derive(Debug, thiserror::Error)]
pub enum TaskerError {
    #[error("Failed to read database file: {0}")]
    ReadError(String),

    #[error("Failed to write database file: {0}")]
    WriteError(String),

    #[error("Failed to parse JSON: {0}")]
    JsonError(String),

    #[error("Invalid ID: {0}")]
    InvalidId(u64),
}