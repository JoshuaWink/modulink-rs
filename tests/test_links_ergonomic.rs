//! Test link behavior in isolation (ergonomic pattern)

use modulink_rs::context::Context;
use modulink_rs::links::Link;
use std::sync::Arc;

fn double_link() -> Link {
    Arc::new(|ctx: Context| Box::pin(async move {
        let val = ctx.get::<i32>("x").unwrap_or(0);
        let ctx = ctx.insert("x", val * 2);
        ctx
    }))
}

#[tokio::test]
async fn test_link_doubles_value() {
    let ctx = Context::new().insert("x", 21);
    let link = double_link();
    let result = link(ctx).await;
    assert_eq!(result.get::<i32>("x"), Some(42));
}
