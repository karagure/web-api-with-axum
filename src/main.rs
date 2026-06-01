//! Minimal Axum web API.
//!
//! Exposes `GET /ping` and returns `404` (headers only) for everything else.

use axum::{Json, Router, http::StatusCode, routing::get};
use serde_json::{Value, json};

/// Application entrypoint: load config, build the router, and serve.
#[tokio::main]
async fn main() {
    let app = Router::new().route("/ping", get(ping)).fallback(not_found);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("failed to bind TCP listener");

    println!("listening on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.expect("server error");
}

/// Health-check endpoint. Always returns `200 OK` with `{"status":"ok"}`.
async fn ping() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

/// Catch-all handler for unknown routes and unsupported methods.
///
/// Returns `404 Not Found` with no body — only headers. This also covers
/// wrong methods on known routes (e.g. `POST /ping` → `404`).
async fn not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}
