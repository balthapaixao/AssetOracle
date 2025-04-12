use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use sqlx::PgPool;
use crate::models::Asset;

#[derive(Deserialize)]
pub struct SimulationInput {
    pub symbol: String,
    pub expected_multiplier: f64,
}

#[derive(serde::Serialize)]
pub struct SimulationResult {
    pub symbol: String,
    pub current_price: f64,
    pub projected_price: f64,
}

/// POST endpoint to simulate a new price based on an expected multiplier.
/// Example JSON payload:
/// {
///   "symbol": "AAPL",
///   "expected_multiplier": 1.1
/// }
#[post("/simulate")]
pub async fn simulate(input: web::Json<SimulationInput>, db_pool: web::Data<PgPool>) -> impl Responder {
    let query = sqlx::query_as::<_, Asset>(
        "SELECT id, symbol, current_price FROM assets WHERE symbol = $1"
    )
    .bind(&input.symbol)
    .fetch_one(db_pool.get_ref())
    .await;

    match query {
        Ok(asset) => {
            let projected_price = asset.current_price * input.expected_multiplier;
            let response = SimulationResult {
                symbol: asset.symbol,
                current_price: asset.current_price,
                projected_price,
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            log::error!("Error during simulation: {}", e);
            HttpResponse::NotFound().body("Asset not found")
        }
    }
}
