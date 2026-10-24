use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Payment or credits required: {0}")]
    PaymentRequired(String),

    #[error("Rate limited: {0}")]
    RateLimited(String),

    #[error("Upstream provider error: {0}")]
    ProviderError(String),

    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

#[derive(Debug, Serialize)]
pub struct ErrorBody {
    pub error: ErrorDetail,
}

#[derive(Debug, Serialize)]
pub struct ErrorDetail {
    pub code: &'static str,
    pub message: String,
    pub request_id: String,
}

pub fn redact_secrets(input: &str) -> String {
    let mut out = input.to_string();
    // Redact any sk-or-v1-... or Bearer tokens if ever present in error strings
    while let Some(idx) = out.find("sk-or-") {
        let end = out[idx..]
            .find(|c: char| c.is_whitespace() || c == '"' || c == '\'' || c == ',')
            .map(|e| idx + e)
            .unwrap_or(out.len());
        out.replace_range(idx..end, "[REDACTED_KEY]");
    }
    out
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let request_id = Uuid::new_v4().to_string();
        let (status, code, raw_msg) = match self {
            AppError::BadRequest(m) => (StatusCode::BAD_REQUEST, "BAD_REQUEST", m),
            AppError::Validation(m) => (StatusCode::UNPROCESSABLE_ENTITY, "VALIDATION_ERROR", m),
            AppError::Unauthorized(m) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", m),
            AppError::Forbidden(m) => (StatusCode::FORBIDDEN, "FORBIDDEN", m),
            AppError::NotFound(m) => (StatusCode::NOT_FOUND, "NOT_FOUND", m),
            AppError::Conflict(m) => (StatusCode::CONFLICT, "CONFLICT", m),
            AppError::PaymentRequired(m) => {
                (StatusCode::PAYMENT_REQUIRED, "INSUFFICIENT_CREDITS", m)
            }
            AppError::RateLimited(m) => (StatusCode::TOO_MANY_REQUESTS, "RATE_LIMITED", m),
            AppError::ProviderError(m) => (StatusCode::BAD_GATEWAY, "PROVIDER_ERROR", m),
            AppError::Database(e) => {
                tracing::error!(request_id = %request_id, "Database error: {}", redact_secrets(&e.to_string()));
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR",
                    "A local database error occurred.".to_string(),
                )
            }
            AppError::Io(e) => {
                tracing::error!(request_id = %request_id, "IO error: {}", redact_secrets(&e.to_string()));
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "IO_ERROR",
                    format!("Local filesystem error: {}", e),
                )
            }
            AppError::Json(e) => {
                tracing::error!(request_id = %request_id, "JSON error: {}", redact_secrets(&e.to_string()));
                (
                    StatusCode::BAD_REQUEST,
                    "JSON_ERROR",
                    format!("JSON parsing error: {}", e),
                )
            }
            AppError::Internal(m) => {
                tracing::error!(request_id = %request_id, "Internal error: {}", redact_secrets(&m));
                (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", m)
            }
        };

        let message = redact_secrets(&raw_msg);
        let body = ErrorBody {
            error: ErrorDetail {
                code,
                message,
                request_id,
            },
        };
        (status, Json(body)).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
