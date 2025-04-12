use serde::{Serialize, Deserialize};

/// Represents an asset record from the database.
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Asset {
    pub id: i32,
    pub symbol: String,
    pub current_price: f64,
}
