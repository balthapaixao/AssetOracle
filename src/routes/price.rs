use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::Asset;

/// GET endpoint to retrieve the current price for a given asset symbol.
/// Example: GET /api/price/AAPL
#[get("/price/{symbol}")]
pub async fn get_price(symbol: web::Path<String>, db_pool: web::Data<PgPool>) -> impl Responder {
    let symbol = symbol.into_inner();

    // Query the database for the asset.
    let result = sqlx::query_as::<_, Asset>(
        "SELECT id, symbol, current_price FROM assets WHERE symbol = $1"
    )
    .bind(&symbol)
    .fetch_one(db_pool.get_ref())
    .await;

    match result {
        Ok(asset) => HttpResponse::Ok().json(asset),
        Err(e) => {
            log::error!("Error fetching asset price: {}", e);
            HttpResponse::NotFound().body("Asset not found")
        }
    }
}
