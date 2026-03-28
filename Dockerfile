# Etapa 1: build
FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Etapa 2: runtime
FROM debian:bookworm-slim
WORKDIR /app

# copia só o binário final
COPY --from=builder /app/target/release/payment_rust_2 .

CMD ["./payment_rust_2"]


