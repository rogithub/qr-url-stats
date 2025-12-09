mod handlers;
mod models;
mod utils;

use axum::{
    routing::{get, post},
    Router,
};
use tower_governor::{
    governor::GovernorConfigBuilder,
    GovernorLayer,
};


use sqlx::sqlite::SqlitePool;
use tower_http::services::ServeDir;
use std::{net::SocketAddr, time::Duration};


#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("sqlite:qr.db")
        .await
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
        .fallback_service(ServeDir::new("static"))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("🚀 Servidor corriendo en http://localhost:3000");
    println!("🚦 Rate limit: 10 requests por minuto");
    
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>()
    ).await.unwrap();
}