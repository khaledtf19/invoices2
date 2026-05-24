use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

#[derive(ts_rs::TS, Error, Debug)]
#[ts(export)]
pub enum ApiError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("User not found")]
    UserNotFound,
    #[error("Invoice not found")]
    InvoiceNotFound,
    #[error("Invalid token")]
    InvalidToken,
    #[error("Token expired")]
    TokenExpired,
    #[error("Password hash error")]
    PasswordHashError(String),

    #[error("Internal server error")]
    #[ts(type = "string")]
    InternalError(#[from] anyhow::Error),

    #[error("Database error")]
    #[ts(type = "string")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Unauthorized")]
    Unauthorized,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, message) = match &self {
            ApiError::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                "INVALID_CREDENTIALS",
                self.to_string(),
            ),
            ApiError::UserAlreadyExists => (
                StatusCode::CONFLICT,
                "USER_ALREADY_EXISTS",
                self.to_string(),
            ),
            ApiError::UserNotFound => (StatusCode::NOT_FOUND, "USER_NOT_FOUND", self.to_string()),
            ApiError::InvoiceNotFound => {
                (StatusCode::NOT_FOUND, "INVOICE_NOT_FOUND", self.to_string())
            }
            ApiError::InvalidToken => (StatusCode::UNAUTHORIZED, "INVALID_TOKEN", self.to_string()),
            ApiError::TokenExpired => (StatusCode::UNAUTHORIZED, "TOKEN_EXPIRED", self.to_string()),
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", self.to_string()),
            ApiError::PasswordHashError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "PASSWORD_HASH_ERROR",
                "Password processing error".to_string(),
            ),
            ApiError::InternalError(e) => {
                tracing::error!("Internal error: {e:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL_ERROR",
                    "Internal server error".to_string(),
                )
            }
            ApiError::DatabaseError(e) => {
                tracing::error!("Database error: {e:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "DATABASE_ERROR",
                    "Database error".to_string(),
                )
            }
        };

        let body = Json(json!({
            "status": "error",
            "data": null,
            "error": {
                "code": code,
                "message": message
            }
        }));

        (status, body).into_response()
    }
}
