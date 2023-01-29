FROM rust:1.66.0

# Create a new directory for the application
RUN mkdir -p /usr/src/trander-rust
WORKDIR /usr/src/trander-rust

# Copy the rest of the application code
COPY . .

# Build the application
RUN cargo build --release

# Expose the port that the application will run on
EXPOSE 8080

# Start the application
CMD ["cargo", "run", "--release"]

