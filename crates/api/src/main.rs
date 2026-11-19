use axum::{routing::get, Json, Router};
use common::AppInfo;
use serde_json::{json, Value};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;

async fn health_handler() -> Json<Value> {
    let app_info = AppInfo::default();
    Json(json!({
        "status": "ok",
        "app": app_info,
    }))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/api/health", get(health_handler))
        .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("API server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind TCP listener");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}
