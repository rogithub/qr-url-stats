# === 1. Etapa de Construcción (Build Stage) ===
FROM rust:slim-nightly as builder
RUN apt-get update && apt-get install -y pkg-config libsqlite3-dev
WORKDIR /app
COPY . .
# La compilación sigue siendo para ARM64
RUN cargo build --release --target aarch64-unknown-linux-gnu 

# === 2. Etapa Final Ligera (Runtime Stage) ===
FROM debian:bullseye-slim

# Instala la librería SQLite y limpia cache
RUN apt-get update && apt-get install -y libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/* # Crea el directorio donde K8s montará el PVC
RUN mkdir -p /data/db 

# Copia el binario ARM64
COPY --from=builder /app/target/aarch64-unknown-linux-gnu/release/qr-url-stats /usr/local/bin/

# ESTABLECE LA VARIABLE DE ENTORNO PARA EL CONTENEDOR (PRODUCCIÓN)
ENV DATABASE_URL=sqlite:/data/db/qr.db 

EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/qr-url-stats"]