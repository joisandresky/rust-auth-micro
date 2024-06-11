use std::sync::Arc;

use crate::infrastructure::data::context::app_ctx::AppCtx;
use axum::{routing::get, Router};
use axum_prometheus::metrics_exporter_prometheus::PrometheusHandle;

use super::{auth_handler::auth_routes, role_handler::role_routes};

pub fn create_router(ctx: Arc<AppCtx>, metric_handle: PrometheusHandle) -> Router<Arc<AppCtx>> {
    Router::new()
        .nest("/api/v1/roles", role_routes())
        .nest("/api/v1/auth", auth_routes(ctx.clone()))
        .route("/metrics", get(|| async move { metric_handle.render() }))
        .route("/health", get(health_check))
        .route("/", get(root))
}

pub async fn root() -> &'static str {
    "🚀 Good, now you are on fire!"
}

pub async fn health_check() -> &'static str {
    "OK"
}
