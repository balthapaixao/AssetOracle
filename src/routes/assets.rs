// src/routes/assets.rs
use actix_web::{post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Deserialize)]
pub struct NewAsset {
    pub symbol: String,
    pub current_price: f64,
}

#[derive(Serialize)]
pub struct CurrentPriceResponse {
    pub id: i32,
    pub symbol: String,
    pub current_price: f64,
}

#[post("/assets")]
pub async fn create_asset(new_asset: web::Json<NewAsset>, pool: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO ASSETS.CURRENT_PRICES (symbol, current_price) VALUES ($1, $2) RETURNING id",
        new_asset.symbol,
        new_asset.current_price
    )
    .fetch_one(pool.get_ref())
    .await;

    match result {
        Ok(record) => {
            let response = CurrentPriceResponse {
                id: record.id,
                symbol: new_asset.symbol.clone(),
                current_price: new_asset.current_price,
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => {
            log::error!("Error creating asset: {}", e);
            HttpResponse::InternalServerError().body("Error creating asset")
        }
    }
}

// Add update and delete endpoints similarly.
