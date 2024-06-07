use std::sync::Arc;

use crate::{domain::models::user_role::UserWithRoles, infrastructure::{data::repositories::user_repository::UserRepository, errors::usecase_error::UsecaseError}};


pub struct UserUsecase {
    user_repo: Arc<UserRepository>,
}

impl UserUsecase {
    pub fn new(user_repo: Arc<UserRepository>) -> Self {
        Self {
            user_repo,
        }
    }

    pub async fn get_by_id_with_roles(&self, id: &String) -> Result<UserWithRoles, UsecaseError> {
        let user = self.user_repo.get_user_by_id_with_roles(id.clone()).await?;

        Ok(user)
    }

}