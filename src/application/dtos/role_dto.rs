use serde::Deserialize;
use validator::Validate;

use crate::domain::models::role::Role;


#[derive(Debug, Deserialize, Validate)]
pub struct CreateOrUpdateRoleReq {
    #[validate(length(min = 3, message = "name must be at least 3 characters"))]
    pub name: String,
    pub description: Option<String>,
}

impl From<&CreateOrUpdateRoleReq> for Role {
    fn from(value: &CreateOrUpdateRoleReq) -> Self {
        Role::new(value.name.clone(), value.description.clone())
    }
}