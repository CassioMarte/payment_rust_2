---
description: Workspace instructions for payment_rust_2 Rust payment service project. Covers build commands, layered architecture, async conventions, error handling patterns, and common pitfalls.
---

# Payment Rust 2 Workspace Instructions

This project is a Rust-based payment service using Actix-web, PostgreSQL, and SQLx. Follow these guidelines for consistent development.

## Build and Test Commands

- **Build**: `cargo build` (debug) or `cargo build --release` (production)
- **Test**: `cargo test` (requires `DATABASE_TEST_URL` env var)
- **Docker**: `docker compose up` to start DB + app

See [1-README.md](1-README.md) for setup, [2-README.md](2-README.md) for Docker Compose.

## Architecture Overview

Layered architecture with clear separation:
- **Models** (`src/model/`): Data structures and DTOs
- **Repository** (`src/repository/`): Database queries with SQLx
- **Service** (`src/service/`): Business logic and validation
- **Handler** (`src/handler/`): HTTP endpoints with Actix extractors
- **Routes** (`src/routes/`): Route registration
- **Validator** (`src/validator/`): Input validation with `validator` crate

See [4-README.md](4-README.md) for models, [5.1-README.md](5.1-README.md) for repository patterns, [6.1-README.md](6.1-README.md) for handlers.

## Coding Conventions

- **Async functions**: Prefix with layer name (e.g., `create_new_payment_service`)
- **Error handling**: Use `Result<T, Box<dyn std::error::Error>>` for business logic
- **Timestamps**: Use `chrono::NaiveDateTime` with `Utc::now().naive_utc()`
- **Enums**: PascalCase variants, derive `Serialize/Deserialize`
- **Validation**: Implement `Validate` trait on DTOs

See [4.1-README.md](4.1-README.md) for validation.

## Common Pitfalls

- Ensure `DATABASE_URL` is set (copy from `.env.example`)
- Use `query_as!` for compile-time SQL verification (requires DB connection at build)
- Avoid double primary keys in `init.sql`
- Fix variable name mismatches (e.g., `id` vs `uuid` in refund logic)
- Handle pool exhaustion in tests with proper cleanup

## Key Files

- [src/main.rs](src/main.rs): App entry point and setup
- [Cargo.toml](Cargo.toml): Dependencies (Actix-web 4, SQLx, Validator, etc.)
- [docker-compose.yml](docker/docker-compose.yml): Services configuration
- [init.sql](docker/init.sql): Database schema

Refer to numbered READMEs for detailed explanations of each component.</content>
<parameter name="filePath">/workspaces/payment_rust_2/.github/copilot-instructions.md