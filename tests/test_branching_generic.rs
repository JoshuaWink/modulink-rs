//! Test branching logic in chains (generic pattern)
use modulink_rs::context::ContextMutable;
use modulink_rs::chains::ChainGeneric;
use modulink_rs::links::LinkGeneric;

type MyContext = ContextMutable;
fn set_error_link_mut() -> LinkGeneric<MyContext> {
    std::sync::Arc::new(|mut ctx: MyContext| Box::pin(async move {
        ctx.insert("error", true);
        ctx
    }))
}
fn handle_error_link_mut() -> LinkGeneric<MyContext> {
    std::sync::Arc::new(|mut ctx: MyContext| Box::pin(async move {
        ctx.insert("handled", true);
        ctx
    }))
}

#[tokio::test]
async fn test_chain_branching_generic() {
    let mut chain = ChainGeneric::<MyContext>::new();
    chain.add_link(set_error_link_mut());
    chain.add_link(handle_error_link_mut());
    chain.connect(0, 1, |ctx| ctx.get::<bool>("error") == Some(true));
    let ctx = MyContext::new();
    let result = chain.run(ctx).await;
    assert_eq!(result.get::<bool>("handled"), Some(true));
}
