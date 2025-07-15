//! Test error propagation through links (generic pattern)
use modulink_rs::context::ContextMutable;
use modulink_rs::chains::ChainGeneric;
use modulink_rs::links::LinkGeneric;

type MyContext = ContextMutable;
fn error_link_mut() -> LinkGeneric<MyContext> {
    std::sync::Arc::new(|_ctx: MyContext| Box::pin(async move {
        panic!("forced error");
    }))
}

#[tokio::test]
#[should_panic(expected = "forced error")]
async fn test_chain_error_propagation_generic() {
    let mut chain = ChainGeneric::<MyContext>::new();
    chain.add_link(error_link_mut());
    let ctx = MyContext::new();
    let _ = chain.run(ctx).await;
}
