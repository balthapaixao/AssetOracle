include ./src/.env
# Makefile for AssetOracle with Colors and DB Shell

# ANSI color codes
GREEN=\033[1;32m
YELLOW=\033[1;33m
BLUE=\033[1;34m
RESET=\033[0m

# Default values for Postgres user & database (overridable by your environment)
# POSTGRES_USER ?= assets_dev
# POSTGRES_DB   ?= assetoracle_db

.PHONY: build up down logs test migrate rebuild db-shell

build:
	@echo -e "$(GREEN)[AssetOracle] Building Docker images...$(RESET)"
	docker-compose build

build-frontend:
	@echo -e "$(GREEN)[AssetOracle] Building Frontend Docker image...$(RESET)"
	docker-compose build frontend

up:
	@echo -e "$(BLUE)[AssetOracle] Starting containers...$(RESET)"
	docker-compose up -d

down:
	@echo -e "$(YELLOW)[AssetOracle] Stopping and removing containers...$(RESET)"
	docker-compose down

logs:
	@echo -e "$(BLUE)[AssetOracle] Tailing logs...$(RESET)"
	docker-compose logs -f

test:
	@echo -e "$(GREEN)[AssetOracle] Running tests...$(RESET)"
	cargo test

migrate:
	@echo -e "$(GREEN)[AssetOracle] Running database migrations...$(RESET)"
	docker-compose run assetoracle sqlx migrate run

rebuild:
	@echo -e "$(YELLOW)[AssetOracle] Rebuilding images without cache...$(RESET)"
	docker-compose build --no-cache

# New target to get a psql shell in the postgres container.
db-shell:
	@echo -e "$(BLUE)[AssetOracle] Opening Postgres shell...$(RESET)"
	docker-compose exec postgres psql -U $(POSTGRES_USER) -d $(POSTGRES_DB)
