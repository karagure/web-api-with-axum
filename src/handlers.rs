//! HTTP handlers and the request-counting middleware.

use axum::{
    Json,
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use serde_json::{Value, json};

use crate::state::AppState;

/// Health-check endpoint. Always returns `200 OK` with `{"status":"ok"}`.
pub async fn ping() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}

/// Returns runtime statistics as JSON: total requests received since startup,
/// uptime in seconds, and the instance identifier.
pub async fn stats(State(state): State<AppState>) -> Json<Value> {
    let total = state.counter.total().await;
    let uptime = state.started_at.elapsed().as_secs();

    Json(json!({
        "total_requests": total,
        "uptime_seconds": uptime,
        "instance_id": state.instance_id,
    }))
}

/// Catch-all handler for unknown routes and unsupported methods.
///
/// Returns `404 Not Found` with no body — only headers. This also covers
/// wrong methods on known routes (e.g. `POST /ping` → `404`).
pub async fn not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}

/// Middleware that increments the request counter on every incoming request,
/// before the request is dispatched to its handler (so `/ping`, `/stats`, and
/// 404s are all counted).
pub async fn count_requests(
    State(state): State<AppState>,
    request: Request,
    next: Next,
) -> Response {
    state.counter.increment().await;
    next.run(request).await
}
