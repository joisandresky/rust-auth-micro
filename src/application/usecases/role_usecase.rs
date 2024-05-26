use std::sync::Arc;

use crate::{application::dtos::role_dto::CreateOrUpdateRoleReq, domain::models::role::Role, infrastructure::data::{errors::usecase_error::UsecaseError, repositories::{repository::Repository, role_repository::RoleRepository}}};

pub struct RoleUsecase {
    role_repo: Arc<RoleRepository>,
}

impl RoleUsecase {
    pub fn new(role_repo: Arc<RoleRepository>) -> Self {
        Self {
            role_repo,
        }
    }

    pub async fn get_all(&self) -> Result<Vec<Role>, UsecaseError> {
        let roles = self.role_repo.get_all().await?;

        Ok(roles)
    }

    pub async fn get_by_id(&self, id: String) -> Result<Role, UsecaseError> {
        let role = self.role_repo.get_by_id(id).await?;

        Ok(role)
    }

    pub async fn create(&self, req: CreateOrUpdateRoleReq) -> Result<Role, UsecaseError> {
        let role = self.role_repo.create(Role::from(&req)).await?;

        Ok(role)
    }

    pub async fn update_by_id(&self, id: String, req: CreateOrUpdateRoleReq) -> Result<Role, UsecaseError> {
        let mut role = self.role_repo.get_by_id(id).await?;

        role.merge(&Role::from(&req));

        let updated_role = self.role_repo.update(role).await?;

        Ok(updated_role)
    }

    pub async fn delete_by_id(&self, id: String) -> Result<(), UsecaseError> {
        let role = self.role_repo.get_by_id(id).await?;

        self.role_repo.delete_by_id(role.id).await?;

        Ok(())
    }
}