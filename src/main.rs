mod config;
mod db;
mod errors;
mod models;
mod routes;
mod data_ingestion;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use env_logger;
use db::init_pool;
use data_ingestion::store_historical_data;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    dotenv().ok();

    let config = config::Config::from_env();
    // print the configuration for debugging
    log::info!("Configuration: {:?}", config);
    // Initialize the database connection pool.
    log::info!("Connecting to database at {}", config.database_url);

    let pool = init_pool(&config.database_url)
        .await
        .expect("Failed to create database pool");

    // (Optional) Trigger historical data ingestion for a given asset (e.g., "AAPL").
    if let Err(e) = store_historical_data(&pool, "AAPL", &config.alpha_vantage_api_key).await {
        log::error!("Error storing historical data: {}", e);
    }

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .configure(routes::init_routes)
    })
    .bind(&config.server_addr)?
    .run()
    .await
}
