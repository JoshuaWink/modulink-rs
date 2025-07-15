//! Test context mutation and retrieval (generic pattern)

use modulink_rs::context::ContextMutable;
use modulink_rs::links::LinkGeneric;

type MyContext = ContextMutable;
fn set_key_link_mut(key: &'static str, value: i32) -> LinkGeneric<MyContext> {
    std::sync::Arc::new(move |mut ctx: MyContext| Box::pin(async move {
        ctx.insert(key, value);
        ctx
    }))
}

#[tokio::test]
async fn test_context_inserts_and_gets_generic() {
    let ctx = MyContext::new();
    let link = set_key_link_mut("foo", 42);
    let ctx = link(ctx).await;
    assert_eq!(ctx.get::<i32>("foo"), Some(42));
}
