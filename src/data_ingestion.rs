use reqwest;
use serde_json::Value;
use sqlx::PgPool;

/// Fetch historical price data for the given symbol from Alpha Vantage.
///
/// # Arguments
///
/// * `symbol` - The asset symbol (e.g., "AAPL").
/// * `api_key` - Your Alpha Vantage API key.
pub async fn fetch_historical_data(symbol: &str, api_key: &str) -> Result<Value, Box<dyn std::error::Error>> {
    let url = format!(
        "https://www.alphavantage.co/query?function=TIME_SERIES_DAILY&symbol={}&outputsize=full&apikey={}",
        symbol, api_key
    );

    let response = reqwest::get(&url).await?;
    let json = response.json::<Value>().await?;
    Ok(json)
}

/// Process the JSON response and store historical prices in the database.
///
/// This function expects the JSON contains a "Time Series (Daily)" object.
/// It inserts the "4. close" price for each date into the `historical_prices` table.
pub async fn store_historical_data(pool: &PgPool, symbol: &str, api_key: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json = fetch_historical_data(symbol, api_key).await?;

    let time_series = json.get("Time Series (Daily)")
        .and_then(|v| v.as_object())
        .ok_or("Unable to find 'Time Series (Daily)' in the response")?;

    for (date_str, daily_data) in time_series {
        let close_price_str = daily_data.get("4. close")
            .and_then(|v| v.as_str())
            .ok_or("Missing close price")?;
        let close_price: f64 = close_price_str.parse()?;

        sqlx::query!(
            r#"
            INSERT INTO historical_prices (symbol, date, close_price)
            VALUES ($1, $2, $3)
            ON CONFLICT (symbol, date) DO NOTHING
            "#,
            symbol,
            date_str, // Ensure the date format matches your table definition.
            close_price
        )
        .execute(pool)
        .await?;
    }

    Ok(())
}
