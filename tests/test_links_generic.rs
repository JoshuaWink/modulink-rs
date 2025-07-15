//! Test link behavior in isolation (generic pattern)

use modulink_rs::context::ContextMutable;

use modulink_rs::links::LinkGeneric;

fn double_link_mut() -> LinkGeneric<ContextMutable> {
    std::sync::Arc::new(|mut ctx: ContextMutable| Box::pin(async move {
        let val = ctx.get::<i32>("x").unwrap_or(0);
        ctx.insert("x", val * 2);
        ctx
    }))
}

#[tokio::test]
async fn test_link_doubles_value_generic() {
    let mut ctx = ContextMutable::new();
    ctx.insert("x", 21);
    let link = double_link_mut();
    let result = link(ctx).await;
    assert_eq!(result.get::<i32>("x"), Some(42));
}
