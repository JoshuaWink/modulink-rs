pub mod http_listener;
pub use http_listener::HttpListener;

use async_trait::async_trait;

/// Trait for sync listeners (blocking triggers, CLI, etc)
pub trait BaseListenerSync: Send + Sync {
    /// Start the listener (sync)
    fn start(&self) -> std::io::Result<()>;
    /// Listener name/type
    fn name(&self) -> &'static str;
}

/// Trait for async listeners (HTTP, SSE, async stdio, etc)
#[async_trait]
pub trait BaseListenerAsync: Send + Sync {
    /// Start the listener (async)
    async fn start(&self) -> std::io::Result<()>;
    /// Listener name/type
    fn name(&self) -> &'static str;
}

// Ergonomic aliases for both sync and async listeners
pub use self::BaseListenerSync as ListenerSync;
pub use self::BaseListenerAsync as ListenerAsync;
