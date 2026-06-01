//! Minimal Axum web API.
//!
//! Routes:
//! - `GET /ping`  → `200` with `{"status":"ok"}`
//! - `GET /stats` → `200` with request count, uptime, and instance id
//! - everything else → `404` (headers only)
//!
//! The listening port is read from `PORT` (default `3000`) and the instance id
//! from `INSTANCE_ID` (default: machine hostname), both via a `.env` file.

mod counter;
mod handlers;
mod state;

use std::env;
use std::sync::Arc;
use std::time::Instant;

use axum::{Router, middleware, routing::get};

use crate::counter::InMemoryCounter;
use crate::handlers::{count_requests, not_found, ping, stats};
use crate::state::AppState;

/// Application entrypoint: load config, build state and router, then serve.
#[tokio::main]
async fn main() {
    // Load variables from a local `.env` file if present.
    dotenvy::dotenv().ok();

    // Read PORT, falling back to 3000 when missing or invalid.
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(3000);

    let state = AppState {
        counter: Arc::new(InMemoryCounter::new()),
        started_at: Instant::now(),
        instance_id: resolve_instance_id(),
    };

    let app = build_app(state);

    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("failed to bind TCP listener");

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.expect("server error");
}

/// Build the router: routes, fallback, and the request-counting middleware.
fn build_app(state: AppState) -> Router {
    Router::new()
        .route("/ping", get(ping))
        .route("/stats", get(stats))
        .fallback(not_found)
        .layer(middleware::from_fn_with_state(
            state.clone(),
            count_requests,
        ))
        .with_state(state)
}

/// Resolve the instance id from `INSTANCE_ID`, falling back to the hostname.
fn resolve_instance_id() -> String {
    env::var("INSTANCE_ID")
        .unwrap_or_else(|_| gethostname::gethostname().to_string_lossy().into_owned())
}
