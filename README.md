# 🔗 QR Tracker

Acortador de URLs con códigos QR y tracking detallado de scans. Construido con Rust, Axum y SQLite.

## ✨ Características

- 🚀 API REST ultra rápida con Axum
- 📱 Generación de códigos QR en formato SVG
- 📊 Tracking detallado de cada scan (IP, User-Agent, timestamp)
- 🌍 Zona horaria de Cancún (UTC-5)
- ✅ Validación robusta de URLs
- 🛡️ Rate limiting (10 requests/minuto por IP)
- 💾 Base de datos SQLite con migraciones

## 🛠️ Tecnologías

- **Rust** - Lenguaje de programación
- **Axum** - Framework web asíncrono
- **SQLx** - Cliente SQL asíncrono con compile-time verification
- **SQLite** - Base de datos embebida
- **tower-governor** - Rate limiting
- **qrcode** - Generación de códigos QR
- **chrono** - Manejo de fechas y zonas horarias

## 📋 Prerequisitos

- Rust 1.70+ ([Instalar Rust](https://rustup.rs/))
- SQLx CLI (opcional, para migraciones):
  ```bash
  cargo install sqlx-cli --no-default-features --features sqlite
  ```

## 🚀 Instalación

1. **Clonar el repositorio**
   ```bash
   git clone <tu-repo>
   cd qr-tracker
   ```

2. **Instalar dependencias**
   ```bash
   cargo build
   ```

3. **Crear la base de datos**
   ```bash
   sqlx database create --database-url sqlite:qr.db
   ```

4. **Ejecutar migraciones**
   ```bash
   sqlx migrate run --database-url sqlite:qr.db
   ```

5. **Iniciar el servidor**
   ```bash
   cargo run
   ```

El servidor estará disponible en `http://localhost:3000`

## 📁 Estructura del proyecto

```
qr-tracker/
├── src/
│   ├── handlers/
│   │   ├── mod.rs
│   │   └── links.rs       # Handlers de la API
│   ├── models.rs          # Estructuras de datos
│   ├── utils.rs           # Validación de URLs
│   └── main.rs            # Punto de entrada
├── migrations/            # Migraciones de base de datos
├── static/
│   └── index.html         # Frontend
├── Cargo.toml
└── qr.db                  # Base de datos SQLite
```

## 🔌 API Endpoints

### POST `/api/shorten`
Crea un link corto con código QR.

**Request:**
```json
{
  "url": "https://example.com"
}
```

**Response:**
```json
{
  "id": "abc12345",
  "short_url": "http://localhost:3000/r/abc12345",
  "qr_svg": "<svg>...</svg>"
}
```

### GET `/r/{id}`
Redirige al URL original y registra el scan.

**Ejemplo:**
```bash
curl http://localhost:3000/r/abc12345
```

## 🗄️ Base de datos

### Tabla `links`
```sql
CREATE TABLE links (
    id TEXT PRIMARY KEY,
    original_url TEXT NOT NULL,
    scans INTEGER DEFAULT 0,
    created_at TEXT NOT NULL
);
```

### Tabla `scans`
```sql
CREATE TABLE scans (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    link_id TEXT NOT NULL,
    ip_address TEXT,
    user_agent TEXT,
    scanned_at TEXT NOT NULL,
    FOREIGN KEY (link_id) REFERENCES links(id)
);
```

## 🔧 Comandos útiles

### Crear nueva migración
```bash
sqlx migrate add nombre_de_migracion
```

Edita el archivo generado en `migrations/` y ejecuta:
```bash
sqlx migrate run --database-url sqlite:qr.db
```

### Consultar datos
```bash
# Ver todos los links
sqlite3 qr.db "SELECT * FROM links;"

# Ver todos los scans
sqlite3 qr.db "SELECT * FROM scans;"

# Ver scans de un link específico
sqlite3 qr.db "SELECT * FROM scans WHERE link_id = 'abc12345';"

# Contar scans por link
sqlite3 qr.db "SELECT link_id, COUNT(*) as total FROM scans GROUP BY link_id;"
```

### Testing con curl

**Crear link:**
```bash
curl -X POST http://localhost:3000/api/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://google.com"}'
```

**Probar rate limiting:**
```bash
for i in {1..15}; do
  curl -X POST http://localhost:3000/api/shorten \
    -H "Content-Type: application/json" \
    -d '{"url": "https://google.com"}' \
    -w "\nStatus: %{http_code}\n"
done
```

## ⚙️ Configuración

### Rate Limiting
Edita en `src/main.rs`:
```rust
GovernorConfigBuilder::default()
    .per_second(60)      // Ventana de tiempo
    .burst_size(10)      // Máximo de requests
```

### Zona horaria
Edita en `src/handlers/links.rs`:
```rust
use chrono_tz::America::Cancun;  // Cambiar según tu ubicación
```

### Puerto del servidor
Edita en `src/main.rs`:
```rust
let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
```

## 🚢 Deploy

### Con Cloudflare Tunnel
```bash
# Instalar cloudflared
brew install cloudflare/cloudflare/cloudflared

# Iniciar tunnel
cloudflared tunnel --url http://localhost:3000
```

### Con Docker (próximamente)
```dockerfile
# Dockerfile
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/qr-tracker /usr/local/bin/
CMD ["qr-tracker"]
```

## 🐛 Troubleshooting

**Error: "no such table: links"**
```bash
sqlx migrate run --database-url sqlite:qr.db
```

**Error: "Address already in use"**
- Otro proceso está usando el puerto 3000
- Cambia el puerto en `main.rs` o mata el proceso:
```bash
lsof -ti:3000 | xargs kill
```

**Error de compilación con tower-governor**
```bash
cargo clean
cargo build
```

## 📝 TODO

- [ ] Dashboard con estadísticas
- [ ] Parseo de User-Agent para identificar dispositivos
- [ ] Geolocalización por IP
- [ ] Expiración automática de links
- [ ] Custom short URLs
- [ ] Autenticación con API keys
- [ ] Tests unitarios e integración

## 📄 Licencia

MIT

## 🤝 Contribuciones

¡Las contribuciones son bienvenidas! Abre un issue o pull request.

---

Hecho con ❤️ y Rust 🦀