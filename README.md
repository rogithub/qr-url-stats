# 🔗 qr-url-stats

Acortador de URLs con códigos QR y tracking detallado de scans. Construido con Rust, Axum y SQLite.

## Live server API (ver detalles para hacer requests)
https://qr.xplaya.com


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

- Rust 1.85+ ([Instalar Rust](https://rustup.rs/))
- SQLx CLI (opcional, para migraciones):
  ```bash
  cargo install sqlx-cli --no-default-features --features sqlite
  ```

## 🚀 Instalación

1. **Clonar el repositorio**
   ```bash
   git clone <tu-repo>
   cd qr-url-stats
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

El servidor estará disponible en `https://qr.xplaya.com`

## 📁 Estructura del proyecto

```
qr-url-stats/
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
  "url": "https://qr.xplaya.com"
}
```

**Response:**
```json
{
  "id": "abc12345",
  "short_url": "https://qr.xplaya.com/r/abc12345",
  "qr_svg": "<svg>...</svg>"
}
```

### GET `/r/{id}`
Redirige al URL original y registra el scan.

**Ejemplo:**
```bash
curl https://qr.xplaya.com/r/abc12345
```

### GET `/{id}`
Obtiene el qr code e información del número de scans.

**Ejemplo:**
```bash
curl https://qr.xplaya.com/abc12345
```

## 🗄️ Base de datos

## 🔧 Comandos útiles

```bash
# Crear archivo vacío
touch qr.db

# Ejecutar el SQL
sqlite3 qr.db < init_db.sql

# Verificar
sqlite3 qr.db ".tables"
sqlite3 qr.db ".schema"
```


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
curl -X POST https://qr.xplaya.com/api/shorten \
  -H "Content-Type: application/json" \
  -d '{"url": "https://xplaya.com"}'
```

**Crear location:**
```bash
curl -X POST https://qr.xplaya.com/api/aYv4-ovn/location \
  -H "Content-Type: application/json" \
  -d '{"lat": 20.6772586, "lon": -87.1131889, "description": "Papelería El Gordo" }'
```

**Probar rate limiting:**
```bash
for i in {1..15}; do
  curl -X POST https://qr.xplaya.com/api/shorten \
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
let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
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