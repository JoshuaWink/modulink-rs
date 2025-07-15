//! Test middleware before/after hooks (ergonomic pattern)

use modulink_rs::context::{Context};
use modulink_rs::chains::{Chain};
use std::sync::{Arc, Mutex};
use modulink_rs::middleware::Middleware;

fn dummy_link() -> Arc<dyn Fn(Context) -> std::pin::Pin<Box<dyn std::future::Future<Output = Context> + Send>> + Send + Sync> {
    Arc::new(|_ctx: Context| Box::pin(async move { _ctx }))
}

struct TestMiddleware {
    pub before_called: Arc<Mutex<bool>>,
    pub after_called: Arc<Mutex<bool>>,
}

impl TestMiddleware {
    fn new() -> Self {
        Self {
            before_called: Arc::new(Mutex::new(false)),
            after_called: Arc::new(Mutex::new(false)),
        }
    }
}

impl Middleware<Context> for TestMiddleware {
    fn before<'a>(&'a self, _ctx: &'a Context) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'a>> {
        let before_called = self.before_called.clone();
        Box::pin(async move {
            *before_called.lock().unwrap() = true;
        })
    }
    fn after<'a>(&'a self, _ctx: &'a Context) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'a>> {
        let after_called = self.after_called.clone();
        Box::pin(async move {
            *after_called.lock().unwrap() = true;
        })
    }
}

#[tokio::test]
async fn test_middleware_hooks() {
    let mut chain = Chain::new();
    chain.add_link(dummy_link());
    let mw = TestMiddleware::new();
    let before = mw.before_called.clone();
    let after = mw.after_called.clone();
    chain.use_middleware(Arc::new(mw));
    let ctx = Context::new();
    let _ = chain.run(ctx).await;
    assert_eq!(*before.lock().unwrap(), true);
    assert_eq!(*after.lock().unwrap(), true);
}
