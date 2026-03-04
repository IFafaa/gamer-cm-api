# syntax=docker/dockerfile:1.7

# Build stage
FROM rust:1.85-slim-bookworm AS builder

WORKDIR /app

# Faster + leaner apt
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Use sparse registry for faster dependency resolution
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

# Copy dependency files first (layer caching)
COPY Cargo.toml Cargo.lock* ./

# Create dummy src to compile deps
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Fix dependency versions compatible with Rust 1.85 (keep behavior)
RUN cargo update home@0.5.12 --precise 0.5.9

# Cache cargo registry + target between builds
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo build --release && rm -rf src

# Copy real source
COPY src ./src
COPY migrations ./migrations
COPY swagger.json ./swagger.json

# SQLx offline cache
COPY .sqlx ./.sqlx
ENV SQLX_OFFLINE=true

# Final build with cache
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/game_gc_rust /app/server
COPY --from=builder /app/migrations ./migrations

ENV PORT=8080
EXPOSE 8080

RUN useradd -r -s /bin/false appuser
USER appuser

CMD ["./server"]
