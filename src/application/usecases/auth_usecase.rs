use std::sync::Arc;

use bcrypt::{hash, DEFAULT_COST};
use sqlx::PgPool;
use tokio::sync::Mutex;
use tracing::info;
use validator::Validate;

use crate::{application::dtos::{auth_dto::{LoginRequest, LoginResponse}, user_dto::CreateUserReq}, domain::models::{user::User, user_role::UserRole}, infrastructure::{data::{repositories::{redis_repository::RedisRepository, role_repository::RoleRepository, user_repository::UserRepository, user_role_repository::UserRoleRepository}, tokenizer::paseto::PasetoMaker}, errors::usecase_error::UsecaseError}};

pub struct AuthUsecase {
    user_repo: Arc<UserRepository>,
    role_repo: Arc<RoleRepository>,
    user_role_repo: Arc<UserRoleRepository>,
    redis_repo: Arc<Mutex<RedisRepository>>,
    paseto_maker: Arc<PasetoMaker>
}

impl AuthUsecase {
    pub fn new(user_repo: Arc<UserRepository>, role_repo: Arc<RoleRepository>, user_role_repo: Arc<UserRoleRepository>, redis_repo: Arc<Mutex<RedisRepository>>, paseto_maker: Arc<PasetoMaker>) -> Self {
        Self {
            user_repo,
            role_repo,
            user_role_repo,
            redis_repo,
            paseto_maker,
        }
    }

    pub async fn authenticate(&self, req: LoginRequest) -> Result<LoginResponse, UsecaseError> {
        let _valid = req.validate()?;

        let data = self
            .user_repo
            .get_user_by_email_with_roles(req.email.clone())
            .await
            .map_err(|err| {
                match err {
                    sqlx::Error::RowNotFound => {
                        UsecaseError::new("Your email isn't associated with any accounts in our system".to_string(), 401, None)
                    }
                    _ => UsecaseError::new(format!("Internal server error: {}", err), 500, None),
                }
            })?;

        let valid_pass = bcrypt::verify(&req.password, &data.user.password)?;
        if !valid_pass {
            return Err(UsecaseError::new("Invalid Password".to_string(), 401, None))
        }

        // check if token already exist for this user so doesn't create a new one
        if let Some(token) = self.redis_repo.lock().await.get_auth_token(&data.user.id).await {
            info!("Token already exist for this user, returning it");
            return Ok(LoginResponse::from(token))
        }

        let role_name = data.roles.first().map(|r| r.name.clone()).unwrap_or_else(|| "USER".to_string());
        let access_token = self.paseto_maker.create_token(&data.user.id, &role_name)
            .map_err(|err| {
                UsecaseError::new(format!("Internal server error: {}", err), 500, None)
            })?;

        let _ = self.redis_repo.lock().await.set_auth_data(&data.user.id, &data, &access_token).await;

        info!("New token created for user with id: {}", data.user.id);
        Ok(LoginResponse::from(access_token))
    }

    pub async fn register(&self, db_pool: &PgPool, req: CreateUserReq) -> Result<User, UsecaseError> {
        let _valid = req.validate()?;

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

    pub async fn logout(&self, user_id: &String) -> Result<bool, UsecaseError> {
        tracing::info!("logout user with id: {}", user_id);
        
        self
            .redis_repo
            .lock()
            .await
            .remove_auth_data(user_id)
            .await?;

        Ok(true)
    }
}