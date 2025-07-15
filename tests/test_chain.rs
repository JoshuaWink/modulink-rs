//! Basic tests for modulink-rs Chain and Link (ergonomic pattern)

use modulink_rs::chains::Chain;
use modulink_rs::context::Context;
use modulink_rs::middleware::logging_middleware;
use std::sync::Arc;

fn test_link() -> Arc<dyn Fn(Context) -> std::pin::Pin<Box<dyn std::future::Future<Output = Context> + Send>> + Send + Sync> {
    Arc::new(|ctx: Context| Box::pin(async move {
        let ctx = ctx.insert("tested", true);
        ctx
    }))
}

#[tokio::test]
async fn test_chain_with_logging() {
    let mut chain = Chain::new();
    chain.add_link(test_link());
    chain.use_middleware(logging_middleware());
    let ctx = Context::new();
    let result = chain.run(ctx).await;
    assert_eq!(result.get::<bool>("tested"), Some(true));
}

// Ergonomic: Chain<Context> alias test
#[tokio::test]
async fn test_chain_with_context() {
    let link = Arc::new(|ctx: Context| {
        Box::pin(async move { ctx.insert("a", 1) }) as std::pin::Pin<Box<dyn std::future::Future<Output = Context> + Send>>
    });
    let mut chain = Chain::new();
    chain.add_link(link);
    let ctx = Context::new();
    let result = chain.run(ctx).await;
    assert_eq!(result.get::<i32>("a"), Some(1));
}
