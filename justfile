# start docker container
run:
    docker-compose up --build
# stop docker container
down:
    docker-compose down
# migration for development
migrate:
	docker-compose exec app ./run_migration.sh dev
# migration for testing
migrate-test:
	docker-compose exec app ./run_migration.sh test

