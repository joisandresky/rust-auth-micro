pub mod api;
pub mod application;
pub mod domain;
pub mod infrastructure;

use std::sync::Arc;

use crate::{
    api::rest::router::create_router,
    infrastructure::data::context::{app_ctx::AppCtx, config::AppConfig},
};
use api::grpc::auth::{auth_proto, GrpcAuthService};
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    HeaderValue, Method,
};
use sqlx::PgPool;
use tokio::signal;
use tonic::transport::Server;
use tower_http::cors::CorsLayer;
use tracing::debug;

pub async fn start_service(
    app_cfg: AppConfig,
    db_pool: PgPool,
    redis_multiplexed_conn: redis::aio::MultiplexedConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    // setup cors
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>()?)
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    // setup app context/state
    let app_ctx = Arc::new(AppCtx::new(
        app_cfg.clone(),
        db_pool,
        redis_multiplexed_conn,
    ));

    // setup router
    let app = create_router(app_ctx.clone())
        .layer(cors)
        .with_state(app_ctx);

    // Run Server
    let addr = format!("0.0.0.0:{}", app_cfg.app_port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    debug!("🚀 {} Started on {}", app_cfg.app_name, addr);

    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal("HTTP/REST Service"))
        .await?;

    Ok(())
}

pub async fn start_grpc(
    app_cfg: AppConfig,
    db_pool: PgPool,
    redis_multiplexed_conn: redis::aio::MultiplexedConnection,
) -> Result<(), Box<dyn std::error::Error>> {
    let app_ctx = Arc::new(AppCtx::new(
        app_cfg.clone(),
        db_pool,
        redis_multiplexed_conn,
    ));

    let addr = "[::1]:50051".parse()?;
    let auth_service = GrpcAuthService::new(app_ctx);

    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(auth_proto::FILE_DESCRIPTOR_SET)
        .build()?;

    debug!("🚀 gRPC service listening on {}", &addr);

    Server::builder()
        .add_service(service)
        .add_service(auth_proto::auth_service_server::AuthServiceServer::new(
            auth_service,
        ))
        .serve_with_shutdown(addr, shutdown_signal("gRPC Service"))
        .await?;

    Ok(())
}

async fn shutdown_signal(svc: &str) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install CTRL+C signal handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install terminate signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            debug!("Got CTRL+C signal on {}", svc);
        }
        _ = terminate => {
            debug!("Got terminate signal on {}", svc);
        }
    }
}
