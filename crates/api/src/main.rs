use axum::{
    extract::State,
    routing::get,
    Json, Router,
};
use common::AppInfo;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tower_http::cors::CorsLayer;

#[derive(Default, Clone)]
struct ApiState {
    session_token: Arc<Mutex<Option<String>>>,
}

#[derive(Deserialize, Serialize)]
struct SessionPayload {
    token: String,
}

async fn health_handler() -> Json<Value> {
    let app_info = AppInfo::default();
    Json(json!({
        "status": "ok",
        "app": app_info,
    }))
}

async fn get_session(State(state): State<ApiState>) -> Json<Value> {
    let token = state.session_token.lock().unwrap().clone();
    Json(json!({ "token": token }))
}

async fn set_session(
    State(state): State<ApiState>,
    Json(payload): Json<SessionPayload>,
) -> Json<Value> {
    *state.session_token.lock().unwrap() = Some(payload.token);
    Json(json!({ "status": "ok" }))
}

async fn clear_session(State(state): State<ApiState>) -> Json<Value> {
    *state.session_token.lock().unwrap() = None;
    Json(json!({ "status": "ok" }))
}

#[tokio::main]
async fn main() {
    let state = ApiState::default();

    let app = Router::new()
        .route("/health", get(health_handler))
        .route("/api/health", get(health_handler))
        .route(
            "/session",
            get(get_session).post(set_session).delete(clear_session),
        )
        .route(
            "/api/session",
            get(get_session).post(set_session).delete(clear_session),
        )
        .layer(CorsLayer::permissive())
        .with_state(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("API server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind TCP listener");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}
