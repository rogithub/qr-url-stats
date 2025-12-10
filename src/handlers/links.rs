use axum::{
    extract::{Path, State, ConnectInfo},
    http::{HeaderMap, StatusCode},
    Json,
    response::Redirect,
};
use chrono::Utc;
use chrono_tz::America::Cancun;
use sqlx::SqlitePool;
use std::net::SocketAddr;
use qrcode::{QrCode, render::svg};

use crate::{config::Config, models::{
    ErrorResponse, Link, LocationRequest, LocationResponse, QrResponse, ShortenRequest, ShortenResponse}, utils::validate_url};

pub async fn get_qr(
    State((pool, config)): State<(SqlitePool, Config)>,
    Path(id): Path<String>,
) -> Result<Json<QrResponse>, (StatusCode, Json<ErrorResponse>)> {
    
    let link_option: Option<Link> = sqlx::query_as::<_, Link>(
        "SELECT id, original_url, scans, created_at FROM links WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(&pool)
    .await
    .map_err(|_| {
        // Aquí convertimos sqlx::Error a tu tipo de error
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { 
            error: "Error interno del servidor".to_string() 
        }))
    })?;

    let link = match link_option {
        Some(link) => link,
        None => return Err((StatusCode::NOT_FOUND, Json(ErrorResponse { 
            error: "QR no encontrado".to_string() 
        }))),
    };

    let short_url = format!("{}/r/{}", config.base_url, id);
    
    let code = QrCode::new(&short_url).expect("Error al generar QR");
    let qr_svg = code
        .render::<svg::Color>()
        .min_dimensions(200, 200)
        .build();
    
    Ok(Json(QrResponse {
        id: link.id.clone(),
        original_url: link.original_url.clone(),
        scans: link.scans,
        created_at: link.created_at.clone(),        
        qr_svg,
    }))
}


pub async fn shorten_url(
    State((pool, config)): State<(SqlitePool, Config)>,
    Json(payload): Json<ShortenRequest>,
) -> Result<Json<ShortenResponse>, (StatusCode, Json<ErrorResponse>)> {
    
    // Validar el URL
    let validated_url = match validate_url(&payload.url) {
        Ok(url) => url,
        Err(msg) => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse { error: msg })
            ));
        }
    };
    
    let id = nanoid::nanoid!(8);
    
    // Obtener timestamp actual en zona horaria de Campeche
    let now_cancun = Utc::now().with_timezone(&Cancun);
    let timestamp = now_cancun.to_rfc3339();
    
    sqlx::query("INSERT INTO links (id, original_url, created_at) VALUES (?, ?, ?)")
        .bind(&id)
        .bind(&validated_url)
        .bind(&timestamp)
        .execute(&pool)
        .await
        .expect("Error al guardar en DB");
    
    let short_url = format!("{}/r/{}", config.base_url, id);
    
    let code = QrCode::new(&short_url).expect("Error al generar QR");
    let qr_svg = code
        .render::<svg::Color>()
        .min_dimensions(200, 200)
        .build();
    
    Ok(Json(ShortenResponse {
        id: id.clone(),
        short_url,
        qr_svg,
    }))
}

pub async fn redirect_handler(
    State((pool, _)): State<(SqlitePool, Config)>,
    Path(id): Path<String>,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Result<Redirect, StatusCode> {
    let result = sqlx::query_as::<_, (String,)>(
        "SELECT original_url FROM links WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        eprintln!("❌ Error DB: {}", e); 
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    let (original_url,) = result.ok_or_else(|| {
        eprintln!("❌ Link no encontrado: {}", id); 
        StatusCode::NOT_FOUND
    })?;
    
    let ip = addr.ip().to_string();
    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("Unknown");

    // Obtener timestamp actual en zona horaria de Campeche
    let now_cancun = Utc::now().with_timezone(&Cancun);
    let timestamp = now_cancun.to_rfc3339();
    
    sqlx::query(
        "INSERT INTO scans (link_id, ip_address, user_agent, scanned_at) VALUES (?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(&ip)
    .bind(user_agent)
    .bind(&timestamp)  // ← Agregar esto
    .execute(&pool)
    .await
    .map_err(|e| {
        eprintln!("❌ Error insertando scan: {}", e);  // ← Agregar log
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    
    sqlx::query("UPDATE links SET scans = scans + 1 WHERE id = ?")
        .bind(&id)
        .execute(&pool)
        .await
        .ok();
    
    println!("✅ Scan registrado: {} → {} desde {} ({})", 
             id, original_url, ip, user_agent);
    
    println!("🔄 Redirigiendo a: {}", original_url); 
    
    let redirect = Redirect::permanent(&original_url);
    println!("✅ Redirect creado correctamente"); 
    
    Ok(redirect)
}


pub async fn register_location(
    State((pool, _)): State<(SqlitePool, Config)>,
    Path(id): Path<String>,
    Json(payload): Json<LocationRequest>,
) -> Result<Json<LocationResponse>, (StatusCode, Json<ErrorResponse>)> {
    // Verificar que el link existe
    let link_exists: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM links WHERE id = ?")
        .bind(&id)
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            eprintln!("Error checking link: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { 
                error: "Error interno del servidor".to_string() 
            }))
        })?;
    
    if link_exists == 0 {
        return Err((StatusCode::NOT_FOUND, Json(ErrorResponse { 
            error: "QR no encontrado".to_string() 
        })));
    }
    
    // Obtener timestamp actual
    let now_cancun = Utc::now().with_timezone(&Cancun);
    let timestamp = now_cancun.to_rfc3339();
    
    // Insertar la ubicación
    let result = sqlx::query(
        "INSERT INTO locations (link_id, lat, lon, description, created_at) 
         VALUES (?, ?, ?, ?, ?)"
    )
    .bind(&id)
    .bind(payload.lat)
    .bind(payload.lon)
    .bind(payload.description.unwrap_or_default())
    .bind(&timestamp)
    .execute(&pool)
    .await
    .map_err(|e| {
        eprintln!("Error inserting location: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { 
            error: "Error al guardar la ubicación".to_string() 
        }))
    })?;
    
    Ok(Json(LocationResponse {
        message: format!("Ubicación registrada para QR {}", id),
        location_id: result.last_insert_rowid(),
    }))
}