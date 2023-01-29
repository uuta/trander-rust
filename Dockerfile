FROM rust:1.66.0

# INFO: it makes faster cargo build
# https://note.com/tkhm_dev/n/n439a4b4b9422
ENV CARGO_BUILD_TARGET_DIR=/tmp/target

# Create a new directory for the application
RUN mkdir -p /usr/src/trander-rust
WORKDIR /usr/src/trander-rust

# Copy the rest of the application code
COPY . .

# Build the application
RUN cargo install cargo-watch && cargo build --release

# Expose the port that the application will run on
EXPOSE 8080

