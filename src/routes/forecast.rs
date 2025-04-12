use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ForecastInput {
    pub symbol: String,
    // Optionally include a field for historical trend data if available.
}

#[derive(serde::Serialize)]
pub struct ForecastResult {
    pub symbol: String,
    pub forecast: Vec<f64>,
}

/// POST endpoint to return a 15-day forecast.
/// Initially still uses dummy values; later, integrate a real forecasting algorithm.
#[post("/forecast")]
pub async fn forecast(input: web::Json<ForecastInput>) -> impl Responder {
    let symbol = input.symbol.clone();
    // TODO: Replace with your forecasting logic
    // For demonstration, we simulate a basic linear trend.
    let base_price = 100.0; // In reality, this should be obtained from historical data.
    let trend = 0.75;      // This is a dummy incremental trend value.
    let forecast = (1..=15)
        .map(|day| base_price + trend * day as f64)
        .collect::<Vec<f64>>();

    let result = ForecastResult { symbol, forecast };
    HttpResponse::Ok().json(result)
}
