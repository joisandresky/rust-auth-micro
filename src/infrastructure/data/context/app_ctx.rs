use std::sync::Arc;

use sqlx::PgPool;

use crate::{application::usecases::role_usecase::RoleUsecase, infrastructure::data::repositories::role_repository::RoleRepository};

use super::config::AppConfig;

#[allow(unused)]
pub struct AppCtx {
    pub cfg: AppConfig,
    pub role_usecase: Arc<RoleUsecase>,
}

impl AppCtx {
    pub fn new(
        cfg: AppConfig,
        db_pool: PgPool,
    ) -> Self {
        // repos
        let role_repo = Arc::new(RoleRepository::new(db_pool.clone()));

        // usecase
        let role_usecase = Arc::new(RoleUsecase::new(role_repo));
        
        Self {
            cfg,
            role_usecase,
        }
    }
}