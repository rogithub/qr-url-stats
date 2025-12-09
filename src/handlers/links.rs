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

use crate::{models::{ShortenRequest, ShortenResponse, ErrorResponse}, utils::validate_url};

pub async fn shorten_url(
    State(pool): State<SqlitePool>,
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
    
    let short_url = format!("http://localhost:3000/r/{}", id);
    
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
    State(pool): State<SqlitePool>,
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
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let (original_url,) = result.ok_or(StatusCode::NOT_FOUND)?;
    
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
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    sqlx::query("UPDATE links SET scans = scans + 1 WHERE id = ?")
        .bind(&id)
        .execute(&pool)
        .await
        .ok();
    
    println!("✅ Scan registrado: {} → {} desde {} ({})", 
             id, original_url, ip, user_agent);
    
    Ok(Redirect::to(&original_url))
}