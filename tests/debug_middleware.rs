use modulink_rs::context::ContextMutable;
use modulink_rs::middleware::Middleware;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub struct DebugMiddleware;

impl Middleware<ContextMutable> for DebugMiddleware {
    fn before<'a>(&'a self, ctx: &'a ContextMutable) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        let ctx = ctx.clone();
        Box::pin(async move {
            println!("[DebugMiddleware] Before: {:?}", ctx);
        })
    }
    fn after<'a>(&'a self, ctx: &'a ContextMutable) -> Pin<Box<dyn Future<Output = ()> + Send + 'a>> {
        let ctx = ctx.clone();
        Box::pin(async move {
            println!("[DebugMiddleware] After: {:?}", ctx);
        })
    }
}
