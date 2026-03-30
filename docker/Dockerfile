# Etapa 1: build
FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Etapa 2: runtime
FROM debian:bookworm-slim
WORKDIR /app

# instala libs de runtime comuns (ajuste conforme seu projeto)
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# copia só o binário final
COPY --from=builder /app/target/release/payment_rust_2 .

CMD ["./payment_rust_2"]


