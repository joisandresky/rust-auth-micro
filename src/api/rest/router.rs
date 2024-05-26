use std::sync::Arc;

use axum::{routing::get, Router};
use crate::infrastructure::data::context::app_ctx::AppCtx;

use super::role_handler::role_routes;

pub fn create_router() -> Router<Arc<AppCtx>> {
    Router::new()
        .merge(role_routes())
        .route("/health", get(health_check))
        .route("/", get(root))
}

pub async fn root() -> &'static str {
    "🚀 Good, now you are on fire!"
}

pub async fn health_check() -> &'static str {
    "OK"
}