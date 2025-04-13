use actix_web::{web, HttpResponse, Responder};
use crate::db;
use serde::Deserialize;
use sqlx::PgPool;
use chrono::NaiveDateTime;

#[derive(Deserialize)]
struct NewPrice {
    asset_name: String,
    price: f64,
    date: NaiveDateTime,
}

async fn add_price(pool: web::Data<PgPool>, info: web::Json<NewPrice>) -> impl Responder {
    match db::insert_historical_price(&pool, &info.asset_name, info.price, info.date).await {
        Ok(_) => HttpResponse::Ok().json("Price added successfully"),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

async fn get_prices(pool: web::Data<PgPool>, asset: web::Path<String>) -> impl Responder {
    match db::get_historical_prices(&pool, &asset).await {
        Ok(prices) => HttpResponse::Ok().json(prices),
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

pub fn init_historical_routes() -> actix_web::Scope {
    web::scope("/historical")
        .route("/{asset_name}", web::get().to(get_prices))
        .route("/", web::post().to(add_price))
}
