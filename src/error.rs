use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::{future::Future, pin::Pin};
use thiserror::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ParseContentType {
    SearchResults,
    SeriesMetadata,
    ChapterList,
    HomePage,
    ImageUrl,
    Json,
    Html,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum BlockType {
    Geographic,
    AgeRestriction,
    Authentication,
    Paywall,
    Maintenance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ErrorSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Error, Clone, Serialize, Deserialize)]
pub enum SourceError {
    #[error("Network connection failed: {message}")]
    NetworkError { message: String, retryable: bool },

    #[error("Request timeout after {timeout:?}")]
    Timeout { timeout: Duration },

    #[error("Rate limited - retry after {retry_after} seconds")]
    RateLimit {
        retry_after: u64,
        message: Option<String>,
    },

    #[error("Authentication required: {reason}")]
    AuthRequired {
        reason: String,
        auth_url: Option<String>,
    },

    #[error("Access forbidden: {reason}")]
    Forbidden { reason: String },

    #[error("Content not found: {resource}")]
    NotFound {
        resource: String,
        suggestion: Option<String>,
    },

    #[error("Failed to parse content: {details}")]
    ParseError {
        details: String,
        content_type: ParseContentType,
        structure_changed: bool,
    },

    #[error("Invalid search query: {reason}")]
    InvalidQuery {
        reason: String,
        suggestions: Vec<String>,
    },

    #[error("Source temporarily unavailable: {reason}")]
    SourceUnavailable {
        reason: String,
        estimated_downtime: Option<Duration>,
    },

    #[error("Source configuration error: {setting}")]
    ConfigurationError {
        setting: String,
        expected: String,
        current: String,
    },

    #[error("Unsupported operation: {operation}")]
    UnsupportedOperation {
        operation: String,
        alternatives: Vec<String>,
    },

    #[error("Invalid pagination: {reason}")]
    InvalidPagination {
        reason: String,
        max_allowed: Option<u32>,
    },

    #[error("Filter validation failed: {filter_name} - {reason}")]
    InvalidFilter {
        filter_name: String,
        reason: String,
        valid_values: Vec<String>,
    },

    #[error("Content blocked: {reason}")]
    ContentBlocked {
        reason: String,
        block_type: BlockType,
    },

    #[error("Image loading failed: {url}")]
    ImageError { url: String, src: Option<String> },

    #[error("Unexpected error: {message}")]
    Unexpected {
        message: String,
        error_code: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorContext {
    pub error: SourceError,
    pub severity: ErrorSeverity,
    pub user_message: String,
    pub suggested_actions: Vec<String>,
    pub should_report: bool,
    pub timestamp: u64,
}

impl SourceError {
    pub fn network(message: impl Into<String>, retryable: bool) -> Self {
        Self::NetworkError {
            message: message.into(),
            retryable,
        }
    }

    pub fn rate_limit(retry_after: u64) -> Self {
        Self::RateLimit {
            retry_after,
            message: None,
        }
    }

    pub fn rate_limit_with_message(retry_after: u64, message: impl Into<String>) -> Self {
        Self::RateLimit {
            retry_after,
            message: Some(message.into()),
        }
    }

    pub fn parse_error(details: impl Into<String>, content_type: ParseContentType) -> Self {
        Self::ParseError {
            details: details.into(),
            content_type,
            structure_changed: false,
        }
    }

    pub fn structure_changed(details: impl Into<String>, content_type: ParseContentType) -> Self {
        Self::ParseError {
            details: details.into(),
            content_type,
            structure_changed: true,
        }
    }

    pub fn not_found_with_suggestion(
        resource: impl Into<String>,
        suggestion: impl Into<String>,
    ) -> Self {
        Self::NotFound {
            resource: resource.into(),
            suggestion: Some(suggestion.into()),
        }
    }

    pub fn is_retryable(&self) -> bool {
        match self {
            Self::NetworkError { retryable, .. } => *retryable,
            Self::Timeout { .. } => true,
            Self::RateLimit { .. } => true,
            Self::SourceUnavailable { .. } => true,
            _ => false,
        }
    }

    pub fn retry_after(&self) -> Option<u64> {
        match self {
            Self::RateLimit { retry_after, .. } => Some(*retry_after),
            Self::Timeout { .. } => Some(5), // Default retry after 5 seconds
            Self::NetworkError {
                retryable: true, ..
            } => Some(3),
            _ => None,
        }
    }

    pub fn to_context(self) -> ErrorContext {
        let (severity, user_message, suggested_actions, should_report) = match &self {
            Self::NetworkError {
                retryable: true, ..
            } => (
                ErrorSeverity::Warning,
                "Connection failed. This might be a temporary network issue.".to_string(),
                vec![
                    "Check your internet connection".to_string(),
                    "Try again in a few moments".to_string(),
                ],
                false,
            ),
            Self::RateLimit { retry_after, .. } => (
                ErrorSeverity::Info,
                format!(
                    "Too many requests. Please wait {} seconds before trying again.",
                    retry_after
                ),
                vec![format!("Wait {} seconds and try again", retry_after)],
                false,
            ),
            Self::NotFound {
                suggestion: Some(suggestion),
                ..
            } => (
                ErrorSeverity::Error,
                "Content not found.".to_string(),
                vec![suggestion.clone()],
                false,
            ),
            Self::ParseError {
                structure_changed: true,
                ..
            } => (
                ErrorSeverity::Critical,
                "The manga source appears to have changed. This extension may need an update."
                    .to_string(),
                vec![
                    "Check for extension updates".to_string(),
                    "Report this issue".to_string(),
                ],
                true,
            ),
            Self::AuthRequired { .. } => (
                ErrorSeverity::Error,
                "Authentication required to access this content.".to_string(),
                vec![
                    "Sign in to your account".to_string(),
                    "Check extension settings".to_string(),
                ],
                false,
            ),
            Self::ContentBlocked {
                block_type: BlockType::Geographic,
                ..
            } => (
                ErrorSeverity::Error,
                "This content is not available in your region.".to_string(),
                vec!["Try a different source".to_string()],
                false,
            ),
            _ => (
                ErrorSeverity::Error,
                "An error occurred while loading content.".to_string(),
                vec!["Try again".to_string(), "Check your connection".to_string()],
                false,
            ),
        };

        ErrorContext {
            error: self,
            severity,
            user_message,
            suggested_actions,
            should_report,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }
}

pub type SourceResult<T> = Result<T, SourceError>;
pub type AsyncSourceResult<T> = Pin<Box<dyn Future<Output = Result<T, SourceError>>>>;

pub trait IntoSourceError {
    fn into_source_error(self) -> SourceError;
}

impl IntoSourceError for reqwest::Error {
    fn into_source_error(self) -> SourceError {
        if self.is_timeout() {
            SourceError::Timeout {
                timeout: Duration::from_secs(30),
            }
        } else if self.is_connect() {
            SourceError::network("Connection failed", true)
        } else {
            SourceError::network(self.to_string(), false)
        }
    }
}

impl IntoSourceError for serde_json::Error {
    fn into_source_error(self) -> SourceError {
        SourceError::parse_error(
            format!("JSON parsing failed: {}", self),
            ParseContentType::Json,
        )
    }
}

impl IntoSourceError for url::ParseError {
    fn into_source_error(self) -> SourceError {
        SourceError::parse_error(
            format!("URL parsing failed: {}", self),
            ParseContentType::ImageUrl,
        )
    }
}
