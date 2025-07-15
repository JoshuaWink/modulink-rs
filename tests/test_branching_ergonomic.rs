//! Test branching logic in chains (ergonomic pattern)

use modulink_rs::context::Context;
use modulink_rs::chains::Chain;
use std::sync::Arc;

fn set_error_link() -> Arc<dyn Fn(Context) -> std::pin::Pin<Box<dyn std::future::Future<Output = Context> + Send>> + Send + Sync> {
    Arc::new(|ctx: Context| Box::pin(async move {
        let ctx = ctx.insert("error", true);
        ctx
    }))
}

fn handle_error_link() -> Arc<dyn Fn(Context) -> std::pin::Pin<Box<dyn std::future::Future<Output = Context> + Send>> + Send + Sync> {
    Arc::new(|ctx: Context| Box::pin(async move {
        let ctx = ctx.insert("handled", true);
        ctx
    }))
}

#[tokio::test]
async fn test_chain_branching() {
    let mut chain = Chain::new();
    chain.add_link(set_error_link());
    chain.add_link(handle_error_link());
    chain.connect(0, 1, |ctx| ctx.get::<bool>("error") == Some(true));
    let ctx = Context::new();
    let result = chain.run(ctx).await;
    assert_eq!(result.get::<bool>("handled"), Some(true));
}
