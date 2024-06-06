pub mod api;
pub mod application;
pub mod domain;
pub mod infrastructure;

use std::sync::Arc;

use crate::{
    api::rest::router::create_router,
    infrastructure::data::context::{app_ctx::AppCtx, config::AppConfig, pg_ctx::get_db_pool},
};
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use envconfig::Envconfig;
use tower_http::cors::CorsLayer;
use tracing::{debug, info};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub async fn start_service() {
    // setup app config
    let app_cfg = AppConfig::init_from_env().expect("Failed to parse environment variables");

    // setup logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rust_auth_micro=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // connect to postgres database
    let db_pool = get_db_pool(&app_cfg.database_url).await;
    info!("Successfully connected to database");

    // connect to redis
    let redis_client = redis::Client::open(app_cfg.redis_url.clone()).unwrap();
    let redis_multiplexed_conn = redis_client.get_multiplexed_async_connection().await.unwrap();
    info!("Successfully connected to redis");

    // setup cors
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    // setup app context/state
    let app_ctx = Arc::new(AppCtx::new(app_cfg.clone(), db_pool, redis_multiplexed_conn));

    // setup router
    let app = create_router().layer(cors).with_state(app_ctx);

    // Run Server
    let addr = format!("0.0.0.0:{}", app_cfg.app_port);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    debug!("🚀 {} Started on {}", app_cfg.app_name, addr);

    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}
