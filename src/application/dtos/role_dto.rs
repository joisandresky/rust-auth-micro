use serde::Deserialize;

use crate::domain::models::role::Role;


#[derive(Debug, Deserialize)]
pub struct CreateOrUpdateRoleReq {
    name: String,
    description: Option<String>,
}

impl From<&CreateOrUpdateRoleReq> for Role {
    fn from(value: &CreateOrUpdateRoleReq) -> Self {
        Role::new(value.name.clone(), value.description.clone())
    }
}