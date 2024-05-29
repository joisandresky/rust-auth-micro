use std::sync::Arc;

use sqlx::PgPool;

use crate::{application::usecases::{auth_usecase::AuthUsecase, role_usecase::RoleUsecase}, infrastructure::data::repositories::{role_repository::RoleRepository, user_repository::UserRepository, user_role_repository::UserRoleRepository}};

use super::config::AppConfig;

#[allow(unused)]
pub struct AppCtx {
    pub cfg: AppConfig,
    pub db_pool: Arc<PgPool>,
    pub role_usecase: Arc<RoleUsecase>,
    pub auth_usecase: Arc<AuthUsecase>,
}

impl AppCtx {
    pub fn new(
        cfg: AppConfig,
        db_pool: PgPool,
    ) -> Self {
        // repos
        let role_repo = Arc::new(RoleRepository::new(db_pool.clone()));
        let user_repo = Arc::new(UserRepository::new(db_pool.clone()));
        let user_role_repo = Arc::new(UserRoleRepository::new(db_pool.clone()));

        // usecases
        let role_usecase = Arc::new(RoleUsecase::new(role_repo.clone()));
        let auth_usecase = Arc::new(AuthUsecase::new(user_repo.clone(), role_repo.clone(), user_role_repo.clone()));        
        
        Self {
            cfg,
            role_usecase,
            auth_usecase,
            db_pool: Arc::new(db_pool),
        }
    }
}