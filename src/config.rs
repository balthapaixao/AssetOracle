use std::env;

/// The application's configuration settings.
#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub server_addr: String,
    pub alpha_vantage_api_key: String,
}

impl Config {
    /// Load configuration from environment variables.
    pub fn from_env() -> Self {
        let postgres_user = env::var("POSTGRES_USER").expect("POSTGRES_USER must be set");
        let postgres_password = env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set");
        let postgres_host = env::var("POSTGRES_HOST").expect("POSTGRES_HOST must be set");
        let postgres_port = env::var("POSTGRES_PORT").expect("POSTGRES_PORT must be set");
        let postgres_db = env::var("POSTGRES_DB").expect("POSTGRES_DB must be set");

        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            postgres_user, postgres_password, postgres_host, postgres_port, postgres_db
        );

        let server_addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
        let alpha_vantage_api_key = env::var("ALPHA_VANTAGE_API_KEY").expect("ALPHA_VANTAGE_API_KEY must be set");

        Self {
            database_url,
            server_addr,
            alpha_vantage_api_key,
        }
    }
}
