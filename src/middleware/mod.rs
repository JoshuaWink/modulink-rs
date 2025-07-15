//! Middleware trait for modulink-rust
//! Trait with async before/after hooks.

use crate::context::Context;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub trait Middleware<T>: Send + Sync {
    fn before<'a>(&'a self, ctx: &'a T) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(async move { let _ = ctx; })
    }
    fn after<'a>(&'a self, ctx: &'a T) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        Box::pin(async move { let _ = ctx; })
    }
}

pub type MiddlewareObj = Arc<dyn Middleware<Context>>;

// Built-in Logging middleware
pub struct LoggingMiddleware;

impl Middleware<Context> for LoggingMiddleware {
    fn before<'a>(&'a self, ctx: &'a Context) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        let ctx = ctx.clone();
        Box::pin(async move {
            println!("[Logging] Before: {:?}", ctx);
        })
    }
    fn after<'a>(&'a self, ctx: &'a Context) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        let ctx = ctx.clone();
        Box::pin(async move {
            println!("[Logging] After: {:?}", ctx);
        })
    }
}

pub fn logging_middleware() -> MiddlewareObj {
    Arc::new(LoggingMiddleware)
}
