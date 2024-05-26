use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn get_db_pool(db_url: &String) -> PgPool {
    PgPoolOptions::new()
        .max_connections(50)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database")
}