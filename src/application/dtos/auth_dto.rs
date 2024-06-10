use crate::{
    api::grpc::auth::auth_proto::{
        LoginRequest as grpcLoginRequest, LoginResponse as grpcLoginResponse, RegisterResponse,
        Role as grpcRole, User as grpcUser, UserWithRoles as grpcUserWithRoles,
    },
    domain::models::{role::Role, user::User, user_role::UserWithRoles},
};
use prost_types::Timestamp;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "please provide valid email"))]
    pub email: String,

    #[validate(length(min = 1, message = "please provide your password"))]
    pub password: String,
}

impl From<grpcLoginRequest> for LoginRequest {
    fn from(value: grpcLoginRequest) -> Self {
        LoginRequest {
            email: value.email,
            password: value.password,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub access_token: String,
}

impl From<String> for LoginResponse {
    fn from(value: String) -> Self {
        LoginResponse {
            access_token: value,
        }
    }
}

impl From<LoginResponse> for grpcLoginResponse {
    fn from(value: LoginResponse) -> Self {
        grpcLoginResponse {
            success: true,
            access_token: value.access_token,
        }
    }
}

impl From<User> for RegisterResponse {
    fn from(value: User) -> Self {
        RegisterResponse {
            success: true,
            user_id: value.id,
        }
    }
}

impl From<User> for grpcUser {
    fn from(value: User) -> Self {
        let email_verified_at = match value.email_verified_at {
            Some(value) => Some(Timestamp {
                seconds: value.and_utc().timestamp(),
                nanos: 0,
            }),
            None => None,
        };

        let last_login = match value.last_login {
            Some(value) => Some(Timestamp {
                seconds: value.and_utc().timestamp(),
                nanos: 0,
            }),
            None => None,
        };

        let deleted_at = match value.deleted_at {
            Some(value) => Some(Timestamp {
                seconds: value.and_utc().timestamp(),
                nanos: 0,
            }),
            None => None,
        };

        grpcUser {
            id: value.id,
            email: value.email,
            email_verified_at: email_verified_at,
            last_login,
            is_active: value.is_active,
            created_at: Some(Timestamp {
                seconds: value.created_at.and_utc().timestamp(),
                nanos: 0,
            }),
            updated_at: Some(Timestamp {
                seconds: value.created_at.and_utc().timestamp(),
                nanos: 0,
            }),
            deleted_at,
        }
    }
}

impl From<Role> for grpcRole {
    fn from(value: Role) -> Self {
        let deleted_at = match value.deleted_at {
            Some(value) => Some(Timestamp {
                seconds: value.and_utc().timestamp(),
                nanos: 0,
            }),
            None => None,
        };

        grpcRole {
            id: value.id,
            name: value.name,
            description: value.description.unwrap().to_string(),
            created_at: Some(Timestamp {
                seconds: value.created_at.and_utc().timestamp(),
                nanos: 0,
            }),
            updated_at: Some(Timestamp {
                seconds: value.updated_at.and_utc().timestamp(),
                nanos: 0,
            }),
            deleted_at: deleted_at,
        }
    }
}

impl From<UserWithRoles> for grpcUserWithRoles {
    fn from(value: UserWithRoles) -> Self {
        grpcUserWithRoles {
            user: Some(value.user.into()),
            roles: value.roles.into_iter().map(|role| role.into()).collect(),
        }
    }
}
