# Inicio do projeto 

- crio o Dockerfile 

````
FROM rust:latest AS builder
WORKDIR /app
copy . .
RUN cargo build --release

FROM debian:boolworm-slim
WORKDIR /app

COPY -from=builder /app/target/realease/payment_rust_2

CMD["./payment_rust_2"]
````


## Rodar o cargo init sem o rust na maquina
- docker run --rm -v $(pwd):/app -w /app rust:latest cargo init

## Rodar o dockerfile
- docker build -t payment_rust_2 .

- docker run payment_rust_2

### Cargo.toml inicial
````
[package]
name = "payment_api"
version = "0.1.0"
edition = "2024"

[dependencies]
actix-web = "4
````

## iniciando a MAIN

````
use actix_web::{get, App, HttpServer, Responder};

#[get("/hello")]
async fn hello() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
````