mod handlers;
mod models;
mod utils;

use axum::{
    Router, http::StatusCode, routing::{get, post}
};
use tower_governor::{
    governor::GovernorConfigBuilder,
    GovernorLayer,
};


use sqlx::sqlite::SqlitePool;
use std::{net::SocketAddr, time::Duration};


#[tokio::main]
async fn main() {
    // Lee la variable de entorno DATABASE_URL.
    // Si no existe (Entorno Local), usa la ruta relativa 'sqlite:qr.db'.
    // Si sí existe (Docker/k3s), usará la ruta absoluta 'sqlite:/data/db/qr.db'.
    let pool = SqlitePool::connect(
        &std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "sqlite:qr.db".to_string())
    ).await
    .expect("No se pudo conectar a la base de datos");

    println!("✅ Conectado a SQLite");

    // Configurar rate limiting
    // 10 requests por minuto por IP
    let governor_conf = Box::new(
        GovernorConfigBuilder::default()
            .per_second(60)  // ventana de 60 segundos
            .burst_size(10)           // máximo 10 requests
            .finish()
            .unwrap()
    );

    let governor_limiter = governor_conf.limiter().clone();
    let interval = Duration::from_secs(60);
    // a separate background task to reset the limiter every interval
    tokio::spawn(async move {
        let mut ticker = tokio::time::interval(interval);
        loop {
            ticker.tick().await;
            governor_limiter.retain_recent();
        }
    });

    let app = Router::new()
        .route("/api/shorten", post(handlers::links::shorten_url))
        .route("/r/{id}", get(handlers::links::redirect_handler))
        .layer(GovernorLayer::new(governor_conf))
        .fallback(|| async {
            (StatusCode::NOT_FOUND, "404 - Not Found")
        })
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    
    println!("🚀 Servidor corriendo en http://localhost:3000");
    println!("🚦 Rate limit: 10 requests por minuto");
    
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>()
    ).await.unwrap();
}