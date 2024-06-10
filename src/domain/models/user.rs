use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    #[serde(skip_deserializing, skip_serializing)]
    pub password: String,
    pub email_verified_at: Option<NaiveDateTime>,
    pub last_login: Option<NaiveDateTime>,
    pub is_active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
}

impl User {
    pub fn new(email: String, password: String) -> Self {
        let id = Uuid::new_v4().to_string();
        let now = Local::now().naive_local();

        Self {
            id,
            email,
            password,
            email_verified_at: None,
            last_login: None,
            is_active: true,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }
}
