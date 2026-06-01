//! Shared application state passed to handlers and middleware.

use std::sync::Arc;
use std::time::Instant;

use crate::counter::CounterStore;

/// State shared across all requests.
///
/// Cloned per-request by Axum; the `Arc` makes clones cheap and ensures every
/// clone shares the same counter.
#[derive(Clone)]
pub struct AppState {
    /// Backing store for the global request counter.
    pub counter: Arc<dyn CounterStore>,
    /// Instant the server started, used to compute uptime.
    pub started_at: Instant,
    /// Identifier for this server instance.
    pub instance_id: String,
}
