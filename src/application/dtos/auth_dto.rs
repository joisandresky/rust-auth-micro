use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "please provide valid email"))]
    pub email: String,

    #[validate(length(min = 1, message = "please provide your password"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub success: bool,
    pub access_token: String,
}

impl From<String> for LoginResponse {
    fn from(value: String) -> Self {
        LoginResponse {
            success: true,
            access_token: value,
        }
    }
}