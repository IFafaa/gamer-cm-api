# Build stage
FROM rust:1.85-slim-bookworm AS builder

WORKDIR /app

# Instalar dependencias do sistema para compilacao
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copiar arquivos de dependencia primeiro (cache de camadas)
COPY Cargo.toml Cargo.lock* ./

# Criar src dummy para compilar dependencias
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Fixar versoes de dependencias compativeis com Rust 1.85
RUN cargo update home@0.5.12 --precise 0.5.9

# Compilar apenas dependencias (sera cacheado se nao mudar)
RUN cargo build --release && rm -rf src

# Copiar codigo fonte real
COPY src ./src
COPY migrations ./migrations
COPY swagger.json ./swagger.json

# DATABASE_URL para verificacao de queries em tempo de compilacao
# Passar via --build-arg DATABASE_URL=postgres://...
ARG DATABASE_URL

# Recompilar com codigo real
RUN touch src/main.rs && cargo build --release

# Runtime stage - imagem minima
FROM debian:bookworm-slim

# Instalar apenas runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copiar binario compilado
COPY --from=builder /app/target/release/game_gc_rust /app/server

# Copiar migrations para rodar na inicializacao
COPY --from=builder /app/migrations ./migrations

# Configurar porta padrao
ENV PORT=8080
EXPOSE 8080

# Executar como usuario nao-root por seguranca
RUN useradd -r -s /bin/false appuser
USER appuser

CMD ["./server"]
