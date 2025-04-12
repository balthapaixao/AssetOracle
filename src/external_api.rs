// src/external_api.rs
use reqwest::Error;
use serde::Deserialize;
use std::env;

/// Struct representing the JSON response expected from the external API.
/// Adjust the field names to match your chosen API.
#[derive(Debug, Deserialize)]
struct ExternalPriceResponse {
    symbol: String,
    price: f64,
}

/// Fetch the asset price from an external API.
/// Replace the URL and query parameters with those required by your chosen data provider.
///
/// # Arguments
///
/// * `symbol` - A string slice that holds the asset symbol (e.g., "AAPL").
///
/// # Returns
///
/// * `Result<f64, Error>` - On success, returns the asset price as f64.
pub async fn fetch_asset_price(symbol: &str) -> Result<f64, Error> {
    // Replace the following URL with your actual API endpoint.
    // For example, if you use a free service that does not require an API key, adapt as needed.
    let api_url = format!("https://api.example.com/price?symbol={}", symbol);

    // If your API requires an API key, you can read it from the environment.
    // let api_key = env::var("EXTERNAL_API_KEY").expect("EXTERNAL_API_KEY must be set");

    // Send the GET request.
    let response = reqwest::get(&api_url).await?;
    let price_data: ExternalPriceResponse = response.json().await?;
    Ok(price_data.price)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_fetch_asset_price() {
        // This is a dummy test. In a real scenario, you might mock the HTTP response.
        // For now, you can print out the result or verify that the function returns an error.
        match fetch_asset_price("AAPL").await {
            Ok(price) => {
                println!("Fetched price: {}", price);
                // You can add assertions if using a known dummy endpoint
            },
            Err(e) => {
                println!("Error fetching price: {}", e);
            }
        }
    }
}
