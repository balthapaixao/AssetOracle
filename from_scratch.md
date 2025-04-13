# AssetOracle from Scratch

AssetOracle is a multi-component project with a Rust backend (using Actix Web and SQLx) and a React frontend for visualizing asset pricing, simulations, and forecasts. This guide covers the setup, build, and run procedures to get the full stack operational.

---

## Table of Contents

1. [Prerequisites](#prerequisites)
2. [Project Structure](#project-structure)
3. [Environment Setup](#environment-setup)
4. [Backend Setup (Rust)](#backend-setup-rust)
5. [Docker & Docker Compose](#docker--docker-compose)
6. [Makefile Commands](#makefile-commands)
7. [Frontend Setup (React)](#frontend-setup-react)
8. [Running the Application](#running-the-application)
9. [Next Steps](#next-steps)

---

## Prerequisites

- **Rust & Cargo:** Install Rust via [rustup](https://rustup.rs/). Use Rust 1.81 or newer.
- **Docker & Docker Compose:** Ensure Docker is installed and Docker Compose is available.
- **Node.js & npm:** (or yarn) for the frontend. Node v14 or newer is recommended.
- **SQLx CLI:** (Optional for offline mode)  
  ```bash
  cargo install sqlx-cli --no-default-features --features postgres
  ```

---

## Project Structure

The project is structured as follows:

```plaintext
AssetOracle/
├── Cargo.toml               # Rust project configuration
├── .env                     # Backend environment variables (Postgres, Alpha Vantage, etc.)
├── Dockerfile               # Multi-stage Docker build for the backend
├── docker-compose.yml       # Defines backend and Postgres services
├── Makefile                 # Convenient targets for building, running, and testing
├── sqlx-data.json           # (Optional) SQLx offline manifest
├── src/                     # Rust backend source code
│   ├── main.rs              # Entry point; starts the server and registers routes
│   ├── config.rs            # Reads environment configuration (database URL, keys)
│   ├── db.rs                # Sets up the Postgres connection pool using SQLx
│   ├── errors.rs            # Custom error definitions and HTTP response mappings
│   ├── data_ingestion.rs    # Functions to fetch & store historical asset data from Alpha Vantage
│   ├── models/              # Domain models (e.g., Asset)
│   │   └── mod.rs           
│   └── routes/              # API endpoints
│       ├── mod.rs           # Registers route modules
│       ├── price.rs         # GET /api/price/{symbol} endpoint
│       ├── simulate.rs      # POST /api/simulate endpoint
│       ├── forecast.rs      # POST /api/forecast endpoint
│       └── assets.rs        # CRUD endpoints for assets
└── assetoracle-frontend/    # React frontend application
    ├── package.json         # Frontend project configuration
    ├── .env                 # Frontend environment file (e.g., REACT_APP_API_BASE_URL)
    ├── public/
    │   └── index.html       # HTML template
    └── src/
        ├── index.js         # React entry point
        ├── App.js           # Main app component
        ├── components/      # Reusable React components
        │   ├── AssetPrice.js       
        │   ├── PriceSimulator.js   
        │   └── ForecastChart.js    
        └── services/
            └── api.js       # Functions to call backend API endpoints
```

---

## Environment Setup

1. **Backend Environment (.env at project root):**

   Create a `.env` file in the project root with:
   ```ini
   ALPHA_VANTAGE_API_KEY=YOUR_ALPHA_VANTAGE_API_KEY
   SERVER_ADDR=127.0.0.1:8080
   POSTGRES_USER=assets_dev
   POSTGRES_PASSWORD=Ass3tSsS_dev
   POSTGRES_HOST=localhost
   POSTGRES_PORT=5432
   POSTGRES_DB=assetoracle_db
   ```

2. **Frontend Environment (.env in assetoracle-frontend folder):**

   Create a `.env` file in the frontend folder:
   ```ini
   REACT_APP_API_BASE_URL=http://localhost:8080/api
   ```

---

## Backend Setup (Rust)

1. **Clone and Enter the Project Directory:**

   ```bash
   git clone <repository_url> AssetOracle
   cd AssetOracle
   ```

2. **Generate SQLx Offline Manifest (optional but recommended):**

   With your database running (e.g., via Docker Compose), run:
   ```bash
   sqlx prepare -- --lib
   ```
   This creates an `sqlx-data.json` file. Commit this file into your repository so that the Docker build can use offline mode.

3. **Build Locally (optional):**

   You can test the backend build and run:
   ```bash
   cargo build --release
   cargo run --release
   ```
   Verify that your API endpoints work as expected using tools like Postman or curl.

---

## Docker & Docker Compose

Your Dockerfile and docker-compose setup allow you to containerize the backend and Postgres.

1. **Dockerfile:**  
   Make sure it sets `SQLX_OFFLINE=1` during the build phase:
   ```dockerfile
   # Stage 1: Build the application
   FROM rust:1.81 as builder
   WORKDIR /app
   COPY . .
   RUN apt-get update && apt-get install -y pkg-config libssl-dev
   ENV SQLX_OFFLINE=1
   RUN cargo build --release

   # Stage 2: Minimal runtime image
   FROM debian:buster-slim
   WORKDIR /app
   COPY --from=builder /app/target/release/asset_oracle .
   EXPOSE 8080
   CMD [\"./asset_oracle\"]
   ```

2. **docker-compose.yml:**  
   This file defines both the backend and the Postgres service:
   ```yaml
   version: \"3.8\"

   services:
     assetoracle:
       build: .
       ports:
         - \"8080:8080\"
       env_file: .env
       environment:
         - DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@postgres:${POSTGRES_PORT}/${POSTGRES_DB}
       depends_on:
         - postgres

     postgres:
       image: postgres:13
       container_name: football_networks
       restart: always
       env_file: .env
       ports:
         - \"5432:5432\"
       volumes:
         - ./data/db/data:/var/lib/postgresql/data
   ```

---

## Makefile Commands

The Makefile provides an easy interface to build and run the project.

- **Build the Docker Images:**
  ```bash
  make build
  ```
- **Start Containers:**
  ```bash
  make up
  ```
- **Stop Containers:**
  ```bash
  make down
  ```
- **View Logs:**
  ```bash
  make logs
  ```
- **Run Tests:**
  ```bash
  make test
  ```
- **Run Database Migrations:**
  ```bash
  make migrate
  ```
- **Rebuild Images Without Cache:**
  ```bash
  make rebuild
  ```
- **Open a Postgres Shell:**
  ```bash
  make db-shell
  ```

---

## Frontend Setup (React)

1. **Navigate to the Frontend Folder:**
   ```bash
   cd assetoracle-frontend
   ```

2. **Install Dependencies:**
   ```bash
   npm install
   ```
   (Ensure your `package.json` includes the proper scripts, for example, \"start\": \"react-scripts start\")

3. **Start the Development Server:**
   ```bash
   npm start
   ```
   Your browser should open at [http://localhost:3000](http://localhost:3000) and the React app will fetch data from `REACT_APP_API_BASE_URL` as defined in the `.env` file.

---

## Running the Application

1. **Start the Backend and Postgres Services:**
   From the project root:
   ```bash
   make up
   ```
2. **Migrate the Database (if needed):**
   ```bash
   make migrate
   ```
3. **Verify the Backend:**
   Use Postman or curl to test endpoints, e.g., `GET http://localhost:8080/api/price/AAPL`.
4. **Start the Frontend:**
   Navigate to the frontend directory and run:
   ```bash
   npm start
   ```
5. **Monitor Logs:**
   To view logs:
   ```bash
   make logs
   ```

---

## Next Steps

- **Enhance the Forecasting Model:**  
  Replace dummy forecast logic with a robust forecasting algorithm, either implemented in Rust or as an external service.
  
- **Improve UI and UX:**  
  Refine frontend components, add better error handling, and polish the overall design.

- **Set Up Automated Tests and CI/CD:**  
  Create unit and integration tests, then integrate CI/CD pipelines for continuous deployment.

- **Schedule Data Ingestion:**  
  Integrate a scheduler to periodically update current prices and historical data.

---

This guide should help you set up and run AssetOracle from scratch. Adjust configuration values and paths as necessary for your environment. If you have any questions or need further modifications, please reach out!
