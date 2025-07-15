use modulink_rs::context::{Context, ContextMutable};
use modulink_rs::chains::{Chain, ChainGeneric};
use modulink_rs::links::{Link, LinkGeneric};
use std::sync::Arc;

#[tokio::test]
async fn test_context_link_insert() {
    let link: Link = Arc::new(|ctx: Context| Box::pin(async move {
        ctx.insert("a", 42)
    }));
    let mut chain = Chain::new();
    chain.add_link(link);
    let ctx = Context::new();
    let result = chain.run(ctx).await;
    assert_eq!(result.get::<i32>("a"), Some(42));
}

#[tokio::test]
async fn test_mutable_context_link_insert() {
    let link: LinkGeneric<ContextMutable> = Arc::new(|mut ctx: ContextMutable| Box::pin(async move {
        ctx.insert("a", 42);
        ctx
    }));
    let mut chain = ChainGeneric::<ContextMutable>::new();
    chain.add_link(link);
    let ctx = ContextMutable::new();
    let result = chain.run(ctx).await;
    assert_eq!(result.get::<i32>("a"), Some(42));
}
