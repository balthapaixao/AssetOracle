# Asset Price Simulation & Forecasting Project

This project aims to simulate probable gains and forecast asset prices (stocks, crypto, currencies) by combining two approaches:
1. **Expectation-Based Price Projection:** A calculation of what an asset's price could be based on an input expected value—without regard for time.
2. **15-Day Forecast Model:** A data-driven, time-series forecast of the asset’s price for the next 15 days.

## Business Objectives

- **Market Data Retrieval:**  
  Retrieve the current price of any asset (stock, cryptocurrency, or currency) from reliable APIs.

- **Dual Prediction Models:**
  - **Expectation-Based Projection:**  
    Calculate the asset's price based on your expected value factor.  
    _Formula Example:_  
    `New Price = Current Price × (Expected Value Factor)`
  
  - **15-Day Forecast Model:**  
    Use historical price data to predict the asset’s price trajectory for the next 15 days.  
    _Possible Methods:_  
    - ARIMA/SARIMA models  
    - Facebook’s Prophet  
    - Advanced machine learning techniques (e.g., LSTM networks)
    
  **Why Two Approaches?**  
  - The expectation-based projection enables what-if scenarios based on subjective input, while the forecast model offers an objective, data-driven prediction that accounts for trends and seasonality.

## Technical Architecture & Components

### Tech Stack
- **Programming Language:**  
  - **Rust:** For data retrieval, processing, and developing the backend API services.
  
- **Database:**  
  - **Postgres:** Containerized using Docker for persistent storage of raw and processed data.

### Data Retrieval and Processing
- **Data Sources:**  
  - Utilize APIs (e.g., Yahoo Finance, Alpha Vantage, CoinGecko) to fetch both current and historical price data.
  
- **Data Pipeline:**
  - Retrieve, clean, and store data in Postgres.
  - Create a data ingestion service in Rust to handle API calls and database insertion efficiently.

### Prediction Algorithms
- **Expectation-Based Projection:**
  - Simple computation based on the current price and an expected multiplier.
  
- **15-Day Forecast Model:**
  - **Steps:**
    1. **Data Collection:** Gather historical price data.
    2. **Preprocessing:** Cleanse and prepare data (handle missing values, normalization, etc.).
    3. **Model Selection:**  
       - Traditional models like ARIMA/SARIMA  
       - Modern approaches using statistical methods or machine learning where appropriate.
    4. **Forecasting:** Train the model to generate predictions for the next 15 days.

### Web Application Development
- **Backend:**
  - Develop the API using Rust frameworks (e.g., [Actix Web](https://actix.rs/)).
  - Endpoints will cover:
    - Current price retrieval.
    - Simulating expectation-based price projections.
    - Running and returning the 15-day forecasts.
  
- **Frontend (Optional for Future Integration):**
  - Can be built using JavaScript (React, Vue.js, or plain HTML/CSS/JS) to provide user interactivity and data visualization.
  - Interfacing with the backend via REST APIs.

### Containerization & Deployment
- **Docker:**
  - Use Docker Compose to orchestrate multiple services:
    - Rust-based backend service.
    - Postgres container.
  - This ensures easy deployment, scalability, and isolation.

### Development Roadmap

#### Phase 1: Requirements & Design
- Finalize data sources and API integrations.
- Define the expected value multiplier and the forecasting model’s parameters.
- Outline the database schema for storing asset details, historical data, and prediction results.

#### Phase 2: Data Retrieval & Storage
- Develop Rust scripts to:
  - Fetch real-time and historical data from selected APIs.
  - Clean and insert this data into the Postgres database.
  
#### Phase 3: Prediction Module Development
- Implement:
  - **Module 1:** Expectation-based price computation.
  - **Module 2:** 15-day forecasting pipeline.
- Validate results via a command-line interface before full integration.

#### Phase 4: Web Application Backend
- Develop REST API endpoints to:
  - Provide current asset prices.
  - Accept user inputs (e.g., expected value factors).
  - Serve simulation and forecast results.
- Containerize the service with Docker.

#### Phase 5: Frontend Development (Future Consideration)
- Build a simple web interface with JavaScript to:
  - Input assumptions.
  - Visualize current prices, expected price targets, and forecast graphs.
- Connect frontend components to the RESTful API.

#### Phase 6: Integration, Testing & Deployment
- End-to-end integration testing including:
  - Data retrieval, processing, storage.
  - Prediction algorithms.
  - API and potential frontend interactions.
- Set up logging, error-handling, and monitoring for production.
- Deploy to your chosen cloud platform ensuring CI/CD pipelines are in place.

---

## Next Steps
1. **Confirm Design:**  
   Review this documentation and adjust based on any additional constraints or desired features.
2. **Kick-off Development:**  
   Begin with Phase 1 by finalizing requirements and planning the data retrieval pipeline.
3. **Iterative Build:**  
   Start developing components incrementally with integration tests to validate end-to-end functionality.

This README provides a roadmap for building the application using Rust and Postgres. Let’s start with the initial setup and data ingestion components, and then move step by step towards full application integration.
