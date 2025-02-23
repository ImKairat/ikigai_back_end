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
