//! The error type of every use case. Framework-agnostic: the HTTP adapter decides how each
//! variant is rendered (`crate::adapters::http::app_error_impl`).

use thiserror::Error;

use crate::domain::validation::ValidationError;

#[derive(Error, Debug)]
pub enum AppError {
    /// The input breaks a business rule (400).
    #[error("{0}")]
    Validation(String),

    /// No (valid) identity (401).
    #[error("{0}")]
    Unauthorized(String),

    /// Identity known, action not allowed (403).
    #[error("{0}")]
    Forbidden(String),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    Conflict(String),

    /// Understood but cannot be processed, e.g. the LLM failed (422).
    #[error("{0}")]
    Unprocessable(String),

    #[error("rate limited")]
    RateLimited { retry_after_secs: u64 },

    /// A uniqueness constraint was hit. Persistence adapters raise it; use cases usually turn
    /// it into a [`AppError::Conflict`] with a meaningful message.
    #[error("unique constraint violated")]
    UniqueViolation,

    #[error("Database error: {0}")]
    Database(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type AppResult<T> = Result<T, AppError>;

impl AppError {
    pub fn unauthenticated() -> Self {
        Self::Unauthorized("Authentication required".into())
    }

    pub fn internal(context: impl std::fmt::Display) -> Self {
        Self::Internal(context.to_string())
    }
}

impl From<ValidationError> for AppError {
    fn from(e: ValidationError) -> Self {
        Self::Validation(e.0)
    }
}
