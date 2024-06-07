use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{role::Role, user::User};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserRole {
    pub id: String,
    pub user_id: String,
    pub role_id: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

impl UserRole {
    pub fn new(
        user_id: String,
        role_id: String,
    ) -> Self {
        let id = Uuid::new_v4().to_string();
        let now = Local::now().naive_local();

        Self {
            id,
            user_id,
            role_id,
            created_at: now,
            updated_at: now,
        }

    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserWithRoles {
    #[serde(flatten)]
    pub user: User,
    pub roles: Vec<Role>,
}