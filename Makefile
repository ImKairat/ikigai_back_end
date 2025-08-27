MAIN_PATH := cmd/web/main.go
BIN_PATH := bin/


run:
	@go run ${MAIN_PATH}

# Description: Makefile for docker-compose
up:
	# Up
	@docker-compose up -d

bup:
	# Build and up
	@docker-compose up --build -d

down:
	# Down
	@docker-compose down

logs:
	# Logs
	@docker-compose logs -f
