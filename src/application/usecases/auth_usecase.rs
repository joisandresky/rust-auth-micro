use std::sync::Arc;

use bcrypt::{hash, DEFAULT_COST};
use sqlx::PgPool;
use validator::Validate;

use crate::{application::dtos::user_dto::CreateUserReq, domain::models::{user::User, user_role::UserRole}, infrastructure::{data::repositories::{role_repository::RoleRepository, user_repository::UserRepository, user_role_repository::UserRoleRepository}, errors::usecase_error::UsecaseError}};

pub struct AuthUsecase {
    user_repo: Arc<UserRepository>,
    role_repo: Arc<RoleRepository>,
    user_role_repo: Arc<UserRoleRepository>,
}

impl AuthUsecase {
    pub fn new(user_repo: Arc<UserRepository>, role_repo: Arc<RoleRepository>, user_role_repo: Arc<UserRoleRepository>) -> Self {
        Self {
            user_repo,
            role_repo,
            user_role_repo,
        }
    }

    pub async fn register(&self, db_pool: &PgPool, req: CreateUserReq) -> Result<User, UsecaseError> {
        let _valid = req.validate().map_err(|err| {
            UsecaseError::from(err)
        })?;

        let mut tx = db_pool.begin().await?;

        let role = self
            .role_repo.get_by_name("USER".to_string())
            .await
            .map_err(|err: sqlx::Error| {
                match err {
                    sqlx::Error::RowNotFound => {
                        UsecaseError::new("Can't find the role with name [USER], can't continue the registration Role not Ready".to_string(), 500, None)
                    },
                    _ => {
                        UsecaseError::from(err)
                    }
                }
            })?;

        // check if email is exist
        let email_exist = self.user_repo.is_email_exist(&req.email).await?;
        if email_exist {
            return Err(UsecaseError::new("User with given email already exist, try to use another one".to_string(), 422, None))
        }

        let mut new_user = User::from(&req);
        new_user.password = hash(&req.password, DEFAULT_COST)?;
        let user = self.user_repo.tx_create(&mut tx, new_user).await?;

        let user_to_role = UserRole::new(user.id.clone(), role.id);
        let _user_role = self.user_role_repo.tx_create(&mut tx, user_to_role).await?;

        tx.commit().await?;

        Ok(user)
    }
}