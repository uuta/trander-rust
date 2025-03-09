# Trander Rust

A backend API service built with Rust for the Trander application, providing travel-related information and services.

## Overview

Trander Rust is a RESTful API service that provides various travel-related functionalities including:

- City information retrieval
- Backpacker data
- Nearby location search
- User settings management

The application is built using Actix Web framework and follows a clean architecture pattern with clear separation of concerns.

## Tech Stack

- **Language**: Rust (2021 edition)
- **Web Framework**: Actix Web 4
- **Database**: MySQL 8.0 with Diesel ORM
- **Authentication**: JWT
- **Containerization**: Docker
- **Logging**: Tracing

## Project Structure

```
src/
├── api/           # External API integrations
├── db/            # Database connection and management
├── error/         # Error handling
├── handler/       # HTTP request handlers
├── middleware/    # Actix middleware (JWT, post-processing)
├── model/         # Data models
├── repository/    # Data access layer
├── schema/        # Database schema definitions
├── service/       # Business logic services
├── use_case/      # Application use cases
└── util/          # Utility functions
```

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Docker and Docker Compose
- MySQL 8.0

### Environment Setup

Create a `.env` file in the project root while referencing `.env.example` file.


### Running with Docker

```bash
# Start the application with Docker Compose
docker-compose up

# The API will be available at http://localhost:8081
```

### Running Locally

```bash
# Install dependencies and build
cargo build

# Run the application
cargo run

# Run tests
cargo test
```

### Database Migrations

```bash
# Run database migrations
./run_migration.sh
```

## API Endpoints

- `GET /` - Health check endpoint
- `GET /cities` - Get city information
- `GET /backpacker` - Get backpacker data
- `GET /near-by-search` - Search for nearby locations
- `GET /settings` - Get user settings

## Development

### Code Formatting

The project uses `rustfmt` for code formatting:

```bash
cargo fmt
```

## License

[License information]

## Contributors

[Contributors information]
