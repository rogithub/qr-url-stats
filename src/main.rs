mod handlers;
mod models;
mod utils;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::sqlite::SqlitePool;
use tower_http::services::ServeDir;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let pool = SqlitePool::connect("sqlite:qr.db")
        .await
        .expect("No se pudo conectar a la base de datos");

    println!("✅ Conectado a SQLite");

    let app = Router::new()
        .route("/api/shorten", post(handlers::links::shorten_url))
        .route("/r/{id}", get(handlers::links::redirect_handler))
        .fallback_service(ServeDir::new("static"))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    
    println!("🚀 Servidor corriendo en http://localhost:3000");
    
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>()
    ).await.unwrap();
}