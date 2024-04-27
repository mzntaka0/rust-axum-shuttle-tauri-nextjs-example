build:
	docker compose build

db:
	docker compose up -d

dev:
	cargo sqlx db create
	cargo sqlx migrate run
	cargo watch -x run

test:
	cargo test
