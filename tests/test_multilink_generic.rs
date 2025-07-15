//! Test multiple links in a chain (generic pattern)
use modulink_rs::context::ContextMutable;
use modulink_rs::chains::ChainGeneric;
use modulink_rs::links::LinkGeneric;

type MyContext = ContextMutable;
fn add_one_link_mut() -> LinkGeneric<MyContext> {
    std::sync::Arc::new(|mut ctx: MyContext| Box::pin(async move {
        let val = ctx.get::<i32>("val").unwrap_or(0);
        ctx.insert("val", val + 1);
        ctx
    }))
}

#[tokio::test]
async fn test_chain_multiple_links_generic() {
    let mut chain = ChainGeneric::<ContextMutable>::new();
    chain.add_link(add_one_link_mut());
    chain.add_link(add_one_link_mut());
    chain.add_link(add_one_link_mut());
    let mut ctx = MyContext::new();
    ctx.insert("val", 0);
    let result = chain.run(ctx).await;
    assert_eq!(result.get::<i32>("val"), Some(3));
}
