use std::sync::Arc;

use sqlx::PgPool;
use tokio::sync::Mutex;

use crate::{application::usecases::{auth_usecase::AuthUsecase, role_usecase::RoleUsecase, user_usecase::UserUsecase}, infrastructure::data::{repositories::{redis_repository::RedisRepository, role_repository::RoleRepository, user_repository::UserRepository, user_role_repository::UserRoleRepository}, tokenizer::paseto::PasetoMaker}};

use super::config::AppConfig;

#[allow(unused)]
pub struct AppCtx {
    pub cfg: AppConfig,
    pub db_pool: Arc<PgPool>,
    pub role_usecase: Arc<RoleUsecase>,
    pub auth_usecase: Arc<AuthUsecase>,
    pub user_usecase: Arc<UserUsecase>,
    pub paseto_maker: Arc<PasetoMaker>,
    pub redis_repo: Arc<Mutex<RedisRepository>>,
}

impl AppCtx {
    pub fn new(
        cfg: AppConfig,
        db_pool: PgPool,
        redis_client: redis::aio::MultiplexedConnection,
    ) -> Self {
        // Paseto Maker
        let paseto_maker = Arc::new(PasetoMaker::new(cfg.clone()));

        // repos
        let redis_repo = Arc::new(Mutex::new(RedisRepository::new(redis_client)));
        let role_repo = Arc::new(RoleRepository::new(db_pool.clone()));
        let user_repo = Arc::new(UserRepository::new(db_pool.clone()));
        let user_role_repo = Arc::new(UserRoleRepository::new(db_pool.clone()));

        // usecases
        let role_usecase = Arc::new(RoleUsecase::new(role_repo.clone()));
        let auth_usecase = Arc::new(AuthUsecase::new(user_repo.clone(), role_repo.clone(), user_role_repo.clone(), redis_repo.clone(), paseto_maker.clone()));
        let user_usecase = Arc::new(UserUsecase::new(user_repo.clone()));
        
        Self {
            cfg,
            role_usecase,
            auth_usecase,
            user_usecase,
            db_pool: Arc::new(db_pool),
            paseto_maker,
            redis_repo,
        }
    }
}