#[tokio::test]
async fn test_chain_macro_generic_success() {
    let mut chain = chain![type = MyContext; validate_input(), transform_input(), enrich_input()];
    chain.use_middleware(Arc::new(DebugMiddleware));
    let mut ctx = MyContext::new();
    ctx.insert("input", "hello");
    let result = chain.run(ctx).await;
    assert_eq!(result.get::<String>("transformed"), Some("HELLO".to_string()));
    assert_eq!(result.get::<String>("enriched"), Some("HELLO-enriched".to_string()));
    assert_eq!(result.get::<String>("error"), None);
}

/// Test core chain composition and execution (generic pattern, using chain![])
use modulink_rs::context::ContextMutable;
use modulink_rs::chains::ChainGeneric;
use modulink_rs::links::LinkGeneric;
use modulink_rs::chain;
mod debug_middleware;
use debug_middleware::DebugMiddleware;
use std::sync::Arc;
use std::pin::Pin;
use std::future::Future;

type MyContext = ContextMutable;

fn validate_input() -> LinkGeneric<MyContext> {
    Arc::new(|mut ctx: MyContext| {
        Box::pin(async move {
            let input: Option<String> = ctx.get("input");
            if input.is_none() {
                ctx.insert("error", "missing input");
            }
            ctx
        }) as Pin<Box<dyn Future<Output = MyContext> + Send>>
    })
}

fn transform_input() -> LinkGeneric<MyContext> {
    Arc::new(|mut ctx: MyContext| {
        Box::pin(async move {
            let input: Option<String> = ctx.get("input");
            if let Some(val) = input {
                ctx.insert("transformed", val.to_uppercase());
            }
            ctx
        }) as Pin<Box<dyn Future<Output = MyContext> + Send>>
    })
}

fn enrich_input() -> LinkGeneric<MyContext> {
    Arc::new(|mut ctx: MyContext| {
        Box::pin(async move {
            let transformed: Option<String> = ctx.get("transformed");
            if let Some(val) = transformed {
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                ctx.insert("enriched", format!("{}-enriched", val));
            }
            ctx
        }) as Pin<Box<dyn Future<Output = MyContext> + Send>>
    })
}

#[tokio::test]
async fn test_chain_generic_success() {
    let mut chain = ChainGeneric::<MyContext>::new();
    chain.add_link(validate_input());
    chain.add_link(transform_input());
    chain.add_link(enrich_input());
    chain.use_middleware(Arc::new(DebugMiddleware));
    let mut ctx = MyContext::new();
    ctx.insert("input", "hello");
    let result = chain.run(ctx).await;
    assert_eq!(result.get::<String>("transformed"), Some("HELLO".to_string()));
    assert_eq!(result.get::<String>("enriched"), Some("HELLO-enriched".to_string()));
    assert_eq!(result.get::<String>("error"), None);
}

#[tokio::test]
async fn test_chain_generic_error() {
    let mut chain = ChainGeneric::<MyContext>::new();
    chain.add_link(validate_input());
    chain.add_link(transform_input());
    chain.add_link(enrich_input());
    chain.use_middleware(Arc::new(DebugMiddleware));
    let ctx = MyContext::new();
    let result = chain.run(ctx).await;
    assert_eq!(result.get::<String>("error"), Some("missing input".to_string()));
    assert_eq!(result.get::<String>("transformed"), None);
    assert_eq!(result.get::<String>("enriched"), None);
}
