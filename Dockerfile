# syntax=docker/dockerfile:1

FROM rust:1.95.0 AS builder
WORKDIR /app

# Instala o target musl para gerar um binário estático puro
RUN rustup target add x86_64-unknown-linux-musl && \
    apt-get update && apt-get install -y musl-tools musl-dev

# Camada de dependencias
COPY Cargo.toml Cargo.lock ./
RUN mkdir -p src && echo "fn main() {}" > src/main.rs
# 👇 Compila as dependências mirando no target estático
RUN cargo build --target x86_64-unknown-linux-musl --release || true

# Build da aplicacao real
COPY src ./src
# 👇 Força o build estático final do seu código
RUN cargo build --target x86_64-unknown-linux-musl --release

# 🌟 AGORA SIM: Sua imagem segura com Distroless
FROM gcr.io/distroless/cc-debian12:nonroot AS runtime
WORKDIR /app

# Copia o binário estático gerado na pasta do target musl
COPY --from=builder /app/target/release/gerencia-gmud-backend /app/gerencia-gmud-backend

EXPOSE 8080
USER nonroot:nonroot
ENTRYPOINT ["/app/gerencia-gmud-backend"]