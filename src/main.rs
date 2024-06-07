use envconfig::Envconfig;
use rust_auth_micro::infrastructure::data::context::{config::AppConfig, pg_ctx::get_db_pool};
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenvy::dotenv().ok();

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
    let redis_client = redis::Client::open(app_cfg.redis_url.clone())?;
    let redis_multiplexed_conn = redis_client.get_multiplexed_tokio_connection().await?;
    info!("Successfully connected to redis");

    // Spawn the HTTP/REST API server
    let cfg_cloned = app_cfg.clone();
    let db_pool_cloned = db_pool.clone();
    let redis_multiplexed_conn_cloned = redis_multiplexed_conn.clone();
    tokio::spawn(async {
        if let Err(e) = rust_auth_micro::start_grpc(cfg_cloned, db_pool_cloned, redis_multiplexed_conn_cloned).await {
            eprintln!("gRPC server error: {:?}", e);
        }
    });
    
    // Start the gRPC server
    rust_auth_micro::start_service(app_cfg, db_pool, redis_multiplexed_conn).await?;

    Ok(())
}
