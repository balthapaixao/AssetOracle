use serde::{Serialize, Deserialize};
use sqlx::types::time::Date;

/// Represents an asset record from the database.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct CurrentPrice {
    pub id: i32,
    pub symbol: String,
    pub current_price: f64,
}

pub struct 
HistoricalPrice {
    pub id: i32,
    pub symbol: String,
    pub date: Date, // Ensure this matches your database date format
    pub close_price: f64,
}