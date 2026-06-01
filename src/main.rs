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

#[cfg(test)]
mod tests {
    use super::*;

    use axum::body::{Body, to_bytes};
    use axum::http::{Request, StatusCode};
    use serde_json::Value;
    use tower::ServiceExt; // for `oneshot`

    fn test_state() -> AppState {
        AppState {
            counter: Arc::new(InMemoryCounter::new()),
            started_at: Instant::now(),
            instance_id: "test-instance".to_string(),
        }
    }

    async fn get(app: &Router, path: &str) -> axum::response::Response {
        app.clone()
            .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn stats_counts_every_request_including_itself() {
        let app = build_app(test_state());

        // Two /ping requests and one 404 — all should be counted.
        assert_eq!(get(&app, "/ping").await.status(), StatusCode::OK);
        assert_eq!(get(&app, "/ping").await.status(), StatusCode::OK);
        assert_eq!(get(&app, "/missing").await.status(), StatusCode::NOT_FOUND);

        // The 4th request (this /stats call) is also counted by the middleware.
        let response = get(&app, "/stats").await;
        assert_eq!(response.status(), StatusCode::OK);

        let body = to_bytes(response.into_body(), usize::MAX).await.unwrap();
        let json: Value = serde_json::from_slice(&body).unwrap();

        assert_eq!(json["total_requests"], 4);
        assert_eq!(json["instance_id"], "test-instance");
        assert!(json["uptime_seconds"].is_u64());
    }
}
