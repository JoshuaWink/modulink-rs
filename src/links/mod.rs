//! Link type for modulink-rust
//! Pure function: async closure, takes Context and returns Context.
//!
//! # Shadowing Policy (Ergonomic APIs)
//! For ergonomic usage (`Link`), always use variable shadowing (e.g., `let ctx = ...`) instead of `mut`.
//! This prevents accidental mutation and is safer for async/concurrent code. See migration plan for details.
//!
//! Example (ergonomic, shadowing):
//! ```rust
//! use modulink_rs::context::Context;
//! let ctx = Context::new();
//! let ctx = ctx.insert("a", 1);
//! let ctx = ctx.insert("b", 2);
//! ```
//!
//! Advanced/generic links may use `mut` for performance, but must document the tradeoff.

use crate::context::Context;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;


/// The generic async link type for any context.
pub type LinkGeneric<C> = Arc<dyn Fn(C) -> Pin<Box<dyn Future<Output = C> + Send>> + Send + Sync>;

/// The ergonomic link type alias for Context.
/// For backward compatibility and ergonomic usage, export as Link.
pub type Link = LinkGeneric<Context>;

// --- Core API Exports ---


// Context is re-exported from crate::context
// Link and LinkErgonomic are defined above
// Chain, Middleware, Listener will be re-exported from their respective modules

pub use crate::chains::Chain;
pub use crate::middleware::Middleware;

// Listener system: ergonomic exports for sync and async listeners
pub use crate::listeners::ListenerSync;
pub use crate::listeners::ListenerAsync;
