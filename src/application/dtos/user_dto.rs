use serde::Deserialize;

use crate::domain::models::user::User;

#[derive(Debug, Deserialize)]
pub struct CreateUserReq {
    pub email: String,
    pub password: String,
}

impl From<&CreateUserReq> for User {
    fn from(value: &CreateUserReq) -> Self {
        User::new(value.email.clone(), value.password.clone())
    }
}