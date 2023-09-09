build:
	docker-compose build

db:
	docker-compose up

dev:
	cargo watch -x run

test:
	cargo test

flow:
	docker-compose build,
	docker-compose up,
	docker exec -it {id} bash,
	psql -U admin todos,
	
