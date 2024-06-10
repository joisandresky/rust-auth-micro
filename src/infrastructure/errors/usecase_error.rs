use axum::{http::StatusCode, response::IntoResponse, Json};
use core::fmt;
use redis::RedisError;
use serde::Serialize;
use serde_json::json;
use sqlx::Error as SqlxError;
use tokio::task::JoinError;
use validator::ValidationErrors;

use super::tokenizer_error::TokenizerError;

#[derive(Debug, Serialize)]
pub struct UsecaseError {
    pub message: String,
    pub error: Option<String>,
    pub code: u16,
}

impl fmt::Display for UsecaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]: {}", self.code, self.message)
    }
}

impl std::error::Error for UsecaseError {}

impl From<SqlxError> for UsecaseError {
    fn from(err: SqlxError) -> Self {
        match err {
            SqlxError::RowNotFound => UsecaseError {
                message: "Resource not found".to_string(),
                error: Some(err.to_string()),
                code: StatusCode::NOT_FOUND.as_u16(),
            },
            SqlxError::Database(err) => UsecaseError {
                message: err.to_string(),
                error: None,
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            },
            _ => UsecaseError {
                message: err.to_string(),
                error: None,
                code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            },
        }
    }
}

impl From<RedisError> for UsecaseError {
    fn from(err: RedisError) -> Self {
        UsecaseError {
            message: err.to_string(),
            error: None,
            code: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        }
    }
}

impl IntoResponse for UsecaseError {
    fn into_response(self) -> axum::response::Response {
        let status_code =
            StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

        let body = json!({
            "message": self.message,
            "error": self.error,
            "success": false,
        });

        (status_code, Json(body)).into_response()
    }
}

impl From<argon2::password_hash::Error> for UsecaseError {
    fn from(value: argon2::password_hash::Error) -> Self {
        UsecaseError::new(value.to_string(), 500, None)
    }
}

impl From<ValidationErrors> for UsecaseError {
    fn from(value: ValidationErrors) -> Self {
        UsecaseError::new(value.to_string(), 400, None)
    }
}

impl From<TokenizerError> for UsecaseError {
    fn from(value: TokenizerError) -> Self {
        match value {
            _ => UsecaseError::new(value.to_string(), 400, None),
        }
    }
}

// Implementing From<JoinError> for UsecaseError
impl From<JoinError> for UsecaseError {
    fn from(err: JoinError) -> Self {
        UsecaseError {
            message: "A blocking task failed to complete".to_string(),
            error: Some(err.to_string()),
            code: 500, // Internal Server Error
        }
    }
}

impl UsecaseError {
    pub fn new(message: String, code: u16, err: Option<String>) -> Self {
        Self {
            message,
            code,
            error: err,
        }
    }
}
