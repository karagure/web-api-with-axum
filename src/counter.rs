//! Request counter storage, isolated behind the [`CounterStore`] trait.

use std::sync::atomic::{AtomicU64, Ordering};

use async_trait::async_trait;

/// Storage for the global request counter.
///
/// Implementations must be cheap to share across requests (used behind an
/// `Arc`). Async methods leave room for future I/O-backed implementations
/// (e.g. Redis) without changing callers.
#[async_trait]
pub trait CounterStore: Send + Sync {
    /// Increment the counter by one.
    async fn increment(&self);
    /// Return the current total.
    async fn total(&self) -> u64;
}

/// In-memory [`CounterStore`] backed by an [`AtomicU64`].
#[derive(Default)]
pub struct InMemoryCounter {
    count: AtomicU64,
}

impl InMemoryCounter {
    /// Create a counter starting at zero.
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl CounterStore for InMemoryCounter {
    async fn increment(&self) {
        self.count.fetch_add(1, Ordering::Relaxed);
    }

    async fn total(&self) -> u64 {
        self.count.load(Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn increments_and_reports_total() {
        let counter = InMemoryCounter::new();
        assert_eq!(counter.total().await, 0);

        for _ in 0..5 {
            counter.increment().await;
        }

        assert_eq!(counter.total().await, 5);
    }
}
