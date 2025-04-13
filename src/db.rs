use sqlx::{PgPool, postgres::PgPoolOptions, Error};
use crate::models::HistoricalPrice;
use chrono::NaiveDate;

/// Initialize a Postgres connection pool.
pub async fn init_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
}


// Insert historical price data
pub async fn insert_historical_price(pool: &PgPool, asset: &str, price: f64, date: NaiveDate) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO ASSETS.HISTORICAL_PRICES (symbol, close_price, date) VALUES ($1, $2, $3)",
        asset, price, date
    )
    .execute(pool)
    .await?;
    Ok(())
}

// Retrieve historical price data
pub async fn get_historical_prices(pool: &PgPool, asset_name: &str) -> Result<Vec<HistoricalPrice>, Error> {
    let prices = sqlx::query_as!(
        HistoricalPrice,
        "SELECT * FROM ASSETS.HISTORICAL_PRICES WHERE symbol = $1 ORDER BY date DESC",
        asset_name
    )
    .fetch_all(pool)
    .await?;
    Ok(prices)
}
