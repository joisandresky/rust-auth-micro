use chrono::{Local, NaiveDateTime};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Debug, Serialize, Deserialize)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>
}

impl Role {
    pub fn new(
        name: String,
        description: Option<String>,
    ) -> Self {
        let id = Uuid::new_v4().to_string();
        let now = Local::now().naive_local();

        Self {
            id,
            name,
            description,
            created_at: now,
            updated_at: now,
            deleted_at: None,
        }
    }

    pub fn update(&mut self, other: &Self) {
        self.name = other.name.clone();
        self.description = other.description.clone();
        self.updated_at = Local::now().naive_local();
    }
}