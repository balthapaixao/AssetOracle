# Stage 1: Build the application
FROM rust:1.81 as builder
WORKDIR /app

# Copy all project files (make sure sqlx-data.json is included)
COPY . .

RUN apt-get update && apt-get install -y pkg-config libssl-dev
# RUN cargo install sqlx-cli --no-default-features --features postgres
# RUN cargo sqlx prepare -- --lib
# Enable SQLx offline mode to use the offline manifest
# ENV SQLX_OFFLINE=1

RUN cargo build --release


# Stage 2: Minimal runtime image
FROM debian:buster-slim
WORKDIR /app
COPY --from=builder /app/target/release/asset_oracle .
EXPOSE 8080
CMD ["./asset_oracle"]
