use modulink_rs::context::ContextMutable;
use modulink_rs::chains::ChainGeneric;
use std::sync::Arc;
use std::future::Future;
use std::pin::Pin;

// Advanced: generics for MutableContext
#[tokio::test]
async fn test_chain_with_mutable_context() {
    let link = Arc::new(|mut ctx: ContextMutable| {
        Box::pin(async move {
            ctx.insert("b", 2);
            ctx
        }) as Pin<Box<dyn Future<Output = ContextMutable> + Send>>
    });
    let mut chain = ChainGeneric::<ContextMutable>::new();
    chain.add_link(link);
    let ctx = ContextMutable::new();
    let result = chain.run(ctx).await;
    assert_eq!(result.get::<i32>("b"), Some(2));
}

// Advanced: generics for custom type
#[derive(Debug, Clone, PartialEq)]
struct CustomCtx(i32);

#[tokio::test]
async fn test_chain_with_custom_type() {
    let link = Arc::new(|ctx: CustomCtx| {
        Box::pin(async move { CustomCtx(ctx.0 + 10) }) as Pin<Box<dyn Future<Output = CustomCtx> + Send>>
    });
    let mut chain = ChainGeneric::<CustomCtx>::new();
    chain.add_link(link);
    let ctx = CustomCtx(5);
    let result = chain.run(ctx).await;
    assert_eq!(result, CustomCtx(15));
}
