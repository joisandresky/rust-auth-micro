use envconfig::Envconfig;


#[derive(Envconfig, Debug, Clone)]
pub struct AppConfig {
    #[envconfig(from = "APP_NAME")]
    pub app_name: String,

    #[envconfig(from = "APP_PORT")]
    pub app_port: u16,

    #[envconfig(from = "APP_ENV")]
    pub app_env: String,

    #[envconfig(from = "DATABASE_URL")]
    pub database_url: String,

    #[envconfig(from = "SECRET_KEY")]
    pub secret_key: String,

    #[envconfig(from = "REDIS_URL")]
    pub redis_url: String,
}