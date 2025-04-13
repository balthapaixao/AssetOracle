# Stage 1: Build the application
FROM rust:1.81 as builder
WORKDIR /app

# Copy project files and dependencies
COPY . .

# Install required libraries to compile (e.g., sqlx with openssl)
RUN apt-get update && apt-get install -y pkg-config libssl-dev ca-certificates

# Set SQLx offline mode
ENV SQLX_OFFLINE=true

# Build the release binary
RUN cargo build --release

# Stage 2: Minimal runtime image
FROM debian:bullseye-slim

# ✅ Install runtime OpenSSL dependency
RUN apt-get update && apt-get install -y libssl1.1 ca-certificates && apt-get clean

WORKDIR /app
COPY --from=builder /app/target/release/asset_oracle .

EXPOSE 8080
CMD ["./asset_oracle"]
