use serde::Deserialize;
use validator::Validate;

use crate::domain::models::user::User;

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUserReq {
    #[validate(email(message = "please provide valid email"))]
    pub email: String,

    #[validate(length(min = 6, message = "password must be at least 6 characters"))]
    pub password: String,
}

impl From<&CreateUserReq> for User {
    fn from(value: &CreateUserReq) -> Self {
        User::new(value.email.clone(), value.password.clone())
    }
}