# === 1. Build Stage ===
FROM --platform=linux/amd64 rust:1.85-bookworm AS builder

# Instalar cross-compiler para ARM64 y dependencias de OpenSSL
RUN apt-get update && \
    apt-get install -y \
        gcc-aarch64-linux-gnu \
        g++-aarch64-linux-gnu \
        pkg-config \
        libsqlite3-dev \
        libssl-dev \
        && \
    rustup target add aarch64-unknown-linux-gnu && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Configurar Cargo para cross-compilation con OpenSSL
RUN mkdir -p .cargo && \
    echo '[target.aarch64-unknown-linux-gnu]' > .cargo/config.toml && \
    echo 'linker = "aarch64-linux-gnu-gcc"' >> .cargo/config.toml && \
    echo 'ar = "aarch64-linux-gnu-ar"' >> .cargo/config.toml

# Configurar variables de entorno para OpenSSL cross-compilation
ENV CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc
ENV CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++
ENV AR_aarch64_unknown_linux_gnu=aarch64-linux-gnu-ar
ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV PKG_CONFIG_PATH=/usr/lib/aarch64-linux-gnu/pkgconfig
ENV OPENSSL_DIR=/usr/include/openssl

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
        libssl3 \
        && \
    rm -rf /var/lib/apt/lists/*

RUN mkdir -p /data/db

COPY --from=builder /app/target/aarch64-unknown-linux-gnu/release/qr-url-stats /usr/local/bin/
COPY --from=builder /app/static /static
COPY --from=builder /app/migrations /migrations

ENV DATABASE_URL=sqlite:/data/db/qr.db

EXPOSE 8080
CMD ["/usr/local/bin/qr-url-stats"]