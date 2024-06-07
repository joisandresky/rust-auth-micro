use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::api::grpc::auth::auth_proto::LoginRequest as grpcLoginRequest;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "please provide valid email"))]
    pub email: String,

    #[validate(length(min = 1, message = "please provide your password"))]
    pub password: String,
}

impl  From<grpcLoginRequest> for LoginRequest {
    fn from(value: grpcLoginRequest) -> Self {
        LoginRequest {
            email: value.email,
            password: value.password,
        }
    }
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