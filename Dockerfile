FROM rust:1.91 AS base

# Install MySQL client for Diesel CLI and migrations
RUN apt-get update && \
    apt-get install -y default-mysql-client && \
    rm -rf /var/lib/apt/lists/*

# INFO: it makes faster cargo build
# https://note.com/tkhm_dev/n/n439a4b4b9422
ENV CARGO_BUILD_TARGET_DIR=/tmp/target

WORKDIR /usr/src/trander-rust

FROM base AS dev

# Tooling needed for local development workflows
RUN cargo install cargo-watch cargo-edit diesel_cli

# Copy the rest of the application code (will be overridden by docker volume mounts in dev)
COPY . .

FROM base AS builder

# Copy source code and build the release binary
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim AS runtime

# Install runtime dependencies for the application
RUN apt-get update && \
    apt-get install -y default-mysql-client && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/trander-rust

# Copy application artifacts from the builder stage
# Only bring the runtime artefacts we actually need
COPY --from=builder /usr/src/trander-rust/migrations ./migrations
COPY --from=builder /tmp/target/release/trander_rust /usr/local/bin/trander-rust

# Fail the build if we accidentally ship source files in the runtime image
RUN test ! -d /usr/src/trander-rust/src

# Expose the port that the application will run on
EXPOSE 8080
