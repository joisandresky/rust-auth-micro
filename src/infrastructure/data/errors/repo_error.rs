use core::fmt;

use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum RepoError {
    NotFound,
    AlreadyExists,
    InternalError(String),
}

impl fmt::Display for RepoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RepoError::NotFound => write!(f, "Resource not found"),
            RepoError::AlreadyExists => write!(f, "Resource already exists"),
            RepoError::InternalError(msg) => write!(f, "{}", msg),
        }
    }
}

impl std::error::Error for RepoError {}