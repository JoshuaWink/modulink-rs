//! Test middleware before/after hooks (generic pattern)

use modulink_rs::context::ContextMutable;
use modulink_rs::chains::ChainGeneric;
use modulink_rs::links::LinkGeneric;
use std::sync::{Arc, Mutex};
use modulink_rs::middleware::Middleware;

type MyContext = ContextMutable;
fn dummy_link_mut() -> LinkGeneric<MyContext> {
    std::sync::Arc::new(|_ctx: MyContext| Box::pin(async move { _ctx }))
}

struct TestMiddlewareMut {
    pub before_called: Arc<Mutex<bool>>,
    pub after_called: Arc<Mutex<bool>>,
}

impl TestMiddlewareMut {
    fn new() -> Self {
        Self {
            before_called: Arc::new(Mutex::new(false)),
            after_called: Arc::new(Mutex::new(false)),
        }
    }
}

impl Middleware<ContextMutable> for TestMiddlewareMut {
    fn before<'a>(&'a self, _ctx: &'a ContextMutable) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'a>> {
        let before_called = self.before_called.clone();
        Box::pin(async move {
            *before_called.lock().unwrap() = true;
        })
    }
    fn after<'a>(&'a self, _ctx: &'a ContextMutable) -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send + 'a>> {
        let after_called = self.after_called.clone();
        Box::pin(async move {
            *after_called.lock().unwrap() = true;
        })
    }
}

#[tokio::test]
async fn test_middleware_hooks_generic() {
    let mut chain = ChainGeneric::<MyContext>::new();
    chain.add_link(dummy_link_mut());
    let mw = TestMiddlewareMut::new();
    let before = mw.before_called.clone();
    let after = mw.after_called.clone();
    chain.use_middleware(Arc::new(mw));
    let ctx = MyContext::new();
    let _ = chain.run(ctx).await;
    assert_eq!(*before.lock().unwrap(), true);
    assert_eq!(*after.lock().unwrap(), true);
}
