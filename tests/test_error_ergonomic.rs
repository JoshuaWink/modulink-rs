//! Test error propagation through links (ergonomic pattern)

use modulink_rs::context::Context;
use modulink_rs::chains::Chain;
use std::sync::Arc;

fn error_link() -> Arc<dyn Fn(Context) -> std::pin::Pin<Box<dyn std::future::Future<Output = Context> + Send>> + Send + Sync> {
    Arc::new(|_ctx: Context| Box::pin(async move {
        panic!("forced error");
    }))
}

#[tokio::test]
#[should_panic(expected = "forced error")]
async fn test_chain_error_propagation() {
    let mut chain = Chain::new();
    chain.add_link(error_link());
    let ctx = Context::new();
    let _ = chain.run(ctx).await;
}
