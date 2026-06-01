//! Minimal Axum web API.
//!
//! Exposes `GET /ping` and returns `404` (headers only) for everything else.
//! The listening port is read from the `PORT` environment variable
//! (via a `.env` file), defaulting to `3000`.

use std::env;

use axum::{Json, Router, http::StatusCode, routing::get};
use serde_json::{Value, json};

/// Application entrypoint: load config, build the router, and serve.
#[tokio::main]
async fn main() {
    // Load variables from a local `.env` file if present.
    dotenvy::dotenv().ok();

    // Read PORT, falling back to 3000 when missing or invalid.
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(3000);

    let app = Router::new().route("/ping", get(ping)).fallback(not_found);

    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind TCP listener");

    println!("listening on {}", listener.local_addr().unwrap());

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
