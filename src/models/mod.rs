use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

/// Represents an asset record from the database.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct CurrentPrice {
    pub id: i32,
    pub symbol: String,
    pub current_price: f64,
}

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug)]
pub struct HistoricalPrice {
    pub id: i32,
    pub symbol: String,
    pub close_price: f64,
    pub date: NaiveDate,
}
