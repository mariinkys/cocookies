//! How application errors look on the wire:
//! `{"status": 404, "message": "...", "timestamp": "..."}` — the shape the frontend expects.
//! Internal details are logged, never sent to the client.

use axum::{
    Json,
    http::{HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::{app_error::AppError, time::now_iso};

#[derive(Serialize)]
struct ErrorBody<'a> {
    status: u16,
    message: &'a str,
    timestamp: String,
}

pub fn status_of(error: &AppError) -> StatusCode {
    match error {
        AppError::Validation(_) => StatusCode::BAD_REQUEST,
        AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
        AppError::Forbidden(_) => StatusCode::FORBIDDEN,
        AppError::NotFound(_) => StatusCode::NOT_FOUND,
        AppError::Conflict(_) | AppError::UniqueViolation => StatusCode::CONFLICT,
        AppError::Unprocessable(_) => StatusCode::UNPROCESSABLE_ENTITY,
        AppError::RateLimited { .. } => StatusCode::TOO_MANY_REQUESTS,
        AppError::Database(_) | AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = status_of(&self);
        let message = match &self {
            AppError::Database(_) | AppError::Internal(_) => {
                // Log the error before it gets converted into a generic response.
                tracing::error!(error = %self, "request failed");
                "An unexpected error occurred"
            }
            AppError::UniqueViolation => "Resource already exists",
            AppError::RateLimited { .. } => "Too many requests, please try again later.",
            AppError::Validation(m)
            | AppError::Unauthorized(m)
            | AppError::Forbidden(m)
            | AppError::NotFound(m)
            | AppError::Conflict(m)
            | AppError::Unprocessable(m) => m.as_str(),
        };

        let body = ErrorBody {
            status: status.as_u16(),
            message,
            timestamp: now_iso(),
        };
        let mut response = (status, Json(body)).into_response();
        if let AppError::RateLimited { retry_after_secs } = self
            && let Ok(v) = HeaderValue::from_str(&retry_after_secs.to_string())
        {
            response.headers_mut().insert(header::RETRY_AFTER, v);
        }
        response
    }
}
