build:
	docker-compose build

db:
	docker-compose up

dev:
	cargo watch -x run

text:
	cargo test