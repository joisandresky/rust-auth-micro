
#[tokio::main]
async fn main() {
    // load environment variables
    dotenvy::dotenv().ok();

    rust_auth_micro::start_service().await;
}
