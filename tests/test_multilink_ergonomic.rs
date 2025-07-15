//! Test multiple links in a chain (ergonomic pattern)

use modulink_rs::context::Context;
use modulink_rs::chains::Chain;
use std::sync::Arc;

fn add_one_link() -> Arc<dyn Fn(Context) -> std::pin::Pin<Box<dyn std::future::Future<Output = Context> + Send>> + Send + Sync> {
    Arc::new(|ctx: Context| Box::pin(async move {
        let val = ctx.get::<i32>("val").unwrap_or(0);
        let ctx = ctx.insert("val", val + 1);
        ctx
    }))
}

#[tokio::test]
async fn test_chain_multiple_links() {
    let mut chain = Chain::new();
    chain.add_link(add_one_link());
    chain.add_link(add_one_link());
    chain.add_link(add_one_link());
    let ctx = Context::new().insert("val", 0);
    let result = chain.run(ctx).await;
    assert_eq!(result.get::<i32>("val"), Some(3));
}
