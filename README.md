## Migration

```sh
# For dev environment
$ docker-compose exec app ./run_migration.sh dev

# For test environment
$ docker-compose exec app ./run_migration.sh test
```

## Production Image Usage

The Dockerfile uses multi-stage builds. The `dev` stage keeps the Rust toolchain for
local workflows, while the `runtime` stage contains only the compiled binary and
the minimal runtime dependencies.

### Build the release image

```sh
docker build --target runtime -t trander-rust:latest .
```

### Run the release container

```sh
docker run --rm \
  --env-file prod.env \
  -p 8080:8080 \
  trander-rust:latest
```

`prod.env` should define the environment variables required by the application,
including the database connection string.

### Apply database migrations in production

The runtime image does not ship with `diesel_cli`. Build the development stage
on the fly and use its `diesel` binary to run migrations against your production
database:

```sh
docker run --rm \
  --env-file prod.env \
  --entrypoint /usr/local/cargo/bin/diesel \
  $(docker build --target dev -q .) migration run
```

Ensure the environment variables in `prod.env` point at the intended MySQL
instance before running this command.
