# === 1. Build Stage ===
FROM --platform=linux/amd64 rust:1.85-bookworm AS builder

# Instalar herramientas de cross-compilation
RUN apt-get update && \
    apt-get install -y \
        gcc-aarch64-linux-gnu \
        pkg-config \
        libsqlite3-dev \
        && \
    rustup target add aarch64-unknown-linux-gnu && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Configurar cross-compilation
RUN mkdir -p .cargo && \
    echo '[target.aarch64-unknown-linux-gnu]' > .cargo/config.toml && \
    echo 'linker = "aarch64-linux-gnu-gcc"' >> .cargo/config.toml

# Copiar manifests y crear dummy project (cachear deps)
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release --target aarch64-unknown-linux-gnu && \
    rm -rf src

# Copiar código real y compilar
COPY . .
RUN touch src/main.rs && \
    cargo build --release --target aarch64-unknown-linux-gnu

# === 2. Runtime Stage ===
FROM --platform=linux/arm64 debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y \
        libsqlite3-0 \
        ca-certificates \
        && \
    rm -rf /var/lib/apt/lists/*

RUN mkdir -p /data/db

# Solo copiar el binario
COPY --from=builder /app/target/aarch64-unknown-linux-gnu/release/qr-url-stats /usr/local/bin/

ENV DATABASE_URL=sqlite:/data/db/qr.db

EXPOSE 3000
CMD ["/usr/local/bin/qr-url-stats"]