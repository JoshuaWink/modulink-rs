//! Test context mutation and retrieval (ergonomic pattern)

use modulink_rs::context::Context;
use std::sync::Arc;

fn set_key_link(key: &'static str, value: i32) -> Arc<dyn Fn(Context) -> std::pin::Pin<Box<dyn std::future::Future<Output = Context> + Send>> + Send + Sync> {
    Arc::new(move |ctx: Context| Box::pin(async move {
        let ctx = ctx.insert(key, value);
        ctx
    }))
}

#[tokio::test]
async fn test_context_inserts_and_gets() {
    let ctx = Context::new();
    let link = set_key_link("foo", 42);
    let ctx = link(ctx).await;
    assert_eq!(ctx.get::<i32>("foo"), Some(42));
}
