use thiserror::Error;

#[derive(Debug, Error)]
pub enum SourceError {
    #[error("Source not found: {0}")]
    NotFound(String),
    #[error("Invalid source URL: {0}")]
    Invalid(String),
    #[error("Network error: {0}")]
    Network(String),
    #[error("Parsing error: {0}")]
    Parsing(String),
    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

pub type SourceResult<T> = Result<T, SourceError>;
