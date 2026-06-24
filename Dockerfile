# syntax=docker/dockerfile:1

FROM rust:1.95.0 AS builder
WORKDIR /app

# Camada de dependencias para acelerar builds subsequentes.
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
RUN cargo build --release || true

# Build da aplicacao.
COPY src ./src
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12:nonroot AS runtime
WORKDIR /app

COPY --from=builder /app/target/release/gerencia-gmud-backend /app/gerencia-gmud-backend

EXPOSE 8080
USER nonroot:nonroot
ENTRYPOINT ["/app/gerencia-gmud-backend"]

