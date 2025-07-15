//! Test core chain composition and execution (ergonomic pattern)

use modulink_rs::context::Context;
use modulink_rs::chains::Chain;
use std::sync::Arc;

fn add_key_link(key: &'static str, value: i32) -> Arc<dyn Fn(Context) -> std::pin::Pin<Box<dyn std::future::Future<Output = Context> + Send>> + Send + Sync> {
    Arc::new(move |ctx: Context| Box::pin(async move {
        let ctx = ctx.insert(key, value);
        ctx
    }))
}

#[tokio::test]
async fn test_chain_composition() {
    let mut chain = Chain::new();
    chain.add_link(add_key_link("a", 1));
    chain.add_link(add_key_link("b", 2));
    let ctx = Context::new();
    let result = chain.run(ctx).await;
    assert_eq!(result.get::<i32>("a"), Some(1));
    assert_eq!(result.get::<i32>("b"), Some(2));
}

#[tokio::test]
async fn test_chain_macro_ergonomic() {
    use modulink_rs::chain;
    use modulink_rs::context::Context;

    use std::sync::Arc;
    // Step 1: Validate input (fail if missing)

    let validate = Arc::new(|ctx: Context| {
        Box::pin(async move {
            let input: Option<String> = ctx.get("input");
            if input.is_none() {
                ctx.insert("error", "missing input")
            } else {
                ctx
            }
        }) as std::pin::Pin<Box<dyn std::future::Future<Output = Context> + Send>>
    });

    let transform = Arc::new(|ctx: Context| {
        Box::pin(async move {
            let input: Option<String> = ctx.get("input");
            match input {
                Some(val) => ctx.insert("transformed", val.to_uppercase()),
                None => ctx,
            }
        }) as std::pin::Pin<Box<dyn std::future::Future<Output = Context> + Send>>
    });

    let enrich = Arc::new(|ctx: Context| {
        Box::pin(async move {
            let transformed: Option<String> = ctx.get("transformed");
            if let Some(val) = transformed {
                tokio::time::sleep(std::time::Duration::from_millis(10)).await;
                ctx.insert("enriched", format!("{}-enriched", val))
            } else {
                ctx
            }
        }) as std::pin::Pin<Box<dyn std::future::Future<Output = Context> + Send>>
    });

    let chain = chain![validate, transform, enrich];
    let ctx = Context::new().insert("input", "hello");
    let result = chain.run(ctx).await;
    assert_eq!(result.get::<String>("transformed"), Some("HELLO".to_string()));
    assert_eq!(result.get::<String>("enriched"), Some("HELLO-enriched".to_string()));
    assert_eq!(result.get::<String>("error"), None);

    // Also test error path
    let ctx = Context::new();
    let result = chain.run(ctx).await;
    assert_eq!(result.get::<String>("error"), Some("missing input".to_string()));
}
