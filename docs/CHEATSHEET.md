### How to Use a Listener (Async and Sync Handlers)
### How to Compose a Chain (Ergonomic Macro)

```rust
use modulink_rs::chain;
use modulink_rs::context::Context;

fn add_key_link(key: &'static str, value: i32) -> impl Fn(Context) -> std::pin::Pin<Box<dyn std::future::Future<Output = Context> + Send>> + Send + Sync {
    move |ctx: Context| Box::pin(async move {
        ctx.insert(key, value)
    })
}

let chain = chain![
    add_key_link("a", 1),
    add_key_link("b", 2),
];
let ctx = Context::new();
let result = chain.run(ctx).await;
assert_eq!(result.get::<i32>("a"), Some(1));
assert_eq!(result.get::<i32>("b"), Some(2));
```

This is the recommended ergonomic pattern for composing async chains.
```rust
use modulink_rs::listener::HttpListener;
use std::sync::Arc;

// Async handler example
struct EchoHandler;
impl EchoHandler {
    fn new() -> Arc<Self> { Arc::new(EchoHandler) }
    fn call(&self, ctx: Context) -> Pin<Box<dyn Future<Output = Context> + Send>> {
        Box::pin(async move {
            let val: Option<String> = ctx.get("input");
            ctx.insert("output", val.unwrap_or_else(|| "none".to_string()))
        })
    }
}
impl Handler for EchoHandler {
    fn call(&self, ctx: Context) -> Pin<Box<dyn Future<Output = Context> + Send>> { self.call(ctx) }
}

// Sync handler example
struct SyncEchoHandler;
impl SyncEchoHandler {
    fn new() -> Arc<Self> { Arc::new(SyncEchoHandler) }
    fn call_sync(&self, ctx: Context) -> Context {
        let val: Option<String> = ctx.get("input");
        ctx.insert("output", val.unwrap_or_else(|| "none".to_string()))
    }
}
impl Handler for SyncEchoHandler {
    fn call(&self, ctx: Context) -> Pin<Box<dyn Future<Output = Context> + Send>> {
        let result = self.call_sync(ctx);
        Box::pin(async move { result })
    }
}

// Create and run an HTTP listener for your handler
let handler = EchoHandler::new();
let listener = HttpListener { handler, addr: "127.0.0.1:8088".to_string() };
tokio::spawn(async move { listener.start().await.unwrap(); });
// ...send requests to http://127.0.0.1:8088/run ...
```

### How to Use the `chain[...]` Macro
```rust
use modulink_rs::chain;
let my_chain = chain[hello_link(), my_link(), another_link()];
```
# ModuLink-RS Cheatsheet

## Shadowing Policy (Ergonomic APIs)
- Always use variable shadowing (e.g., `let ctx = ...`) instead of `mut` for context updates in ergonomic APIs.
- This prevents accidental mutation and is safer for async/concurrent code.
- Advanced/generic APIs may use `mut` for performance, but must document the tradeoff.

## Advanced Usage (very brief)

- For custom or mutable context, use generics:
  - `Chain<ContextMutable>` or `Chain<MyCustomType>`
  - `Link<ContextMutable>` or `Link<MyCustomType>`
- Only use this for advanced scenarios (performance, API compatibility, or custom data models).

Example:
```rust
use modulink_rs::{ContextMutable, Chain};
let mut chain = Chain::<ContextMutable>::new();
```

---


## Ergonomic Usage (recommended for 90% of cases)

### Minimal "Hello, World" Quickstart
---

### Minimal Imports
```rust
use modulink_rs::{Context, Chain};
use std::sync::Arc;
```

### How to Define a Link
```rust
fn my_link() -> Arc<dyn Fn(Context) -> std::pin::Pin<Box<dyn std::future::Future<Output = Context> + Send>> + Send + Sync> {
    Arc::new(|ctx: Context| Box::pin(async move {
        let ctx = ctx.insert("key", "value");
        ctx
    }))
}
```

### How to Add Middleware
```rust
use modulink_rs::middleware::LoggingMiddleware;
chain.use_middleware(LoggingMiddleware::default());
```

### Recommended File Structure
```
src/
  main.rs
  links/
    hello.rs
    validate.rs
  middleware/
    logging.rs
```

### Error Handling / Branching Example
```rust
// chain.connect(source_index, target_index, |ctx| ctx.get::<bool>("should_branch").unwrap_or(false));
```
```rust
// src/links/hello.rs
use modulink_rs::Context;
use std::sync::Arc;

pub fn hello_link() -> Arc<dyn Fn(Context) -> std::pin::Pin<Box<dyn std::future::Future<Output = Context> + Send>> + Send + Sync> {
    Arc::new(|ctx: Context| Box::pin(async move {
        let ctx = ctx.insert("message", "Hello, World!"); // shadowing
        ctx
    }))
}

// src/main.rs
use modulink_rs::{Context, Chain};
use modulink_rs::middleware::LoggingMiddleware;
use crate::links::hello_link;

#[tokio::main]
async fn main() {
    let mut chain = Chain::new();
    chain.add_link(hello_link());
    // Example: add middleware for logging
    chain.use_middleware(LoggingMiddleware::default());
    // Example: connect for conditional branching (optional)
    // chain.connect(0, 1, |ctx| ctx.get::<bool>("should_branch").unwrap_or(false));
    let ctx = Context::new();
    let result = chain.run(ctx).await;
    println!("message: {:?}", result.get::<String>("message"));
}
```

**Best Practice:**
- Place each link in its own file/module (e.g., `src/links/hello.rs`) for testability and clarity.
- Keep chains, links, and middleware in separate files for modularity and easier testing.
- Use shadowing (`let ctx = ctx.insert(...)`) for all context updates.

---

- Use `Context` (immutable) for all data flow.
- Use `Chain` and `Link` (`Context`) for simple, safe, and predictable pipelines.
- Attach middleware and branching as needed.

## Mut vs. Shadowing
- **Shadowing (recommended for ergonomic APIs):**
  ```rust
  let ctx = ctx.insert("a", 1);
  let ctx = ctx.insert("b", 2);
  ```
- **Mut (allowed only in advanced/generic APIs):**
  ```rust
  let mut ctx = ...;
  ctx.insert("a", 1);
  ctx.insert("b", 2);
  ```

## Refresher (Ergonomic)
- Use `Context` for all data flow (immutable, shadowing pattern).
- Use `Chain` and `Link` for simple, safe, and predictable pipelines.
- Always prefer variable shadowing: `let ctx = ctx.insert(...)`.
- Attach middleware for logging, metrics, or side effects as needed.
- Use `.connect()` for conditional branching if required.
- Keep links pure and stateless for maximum safety and testability.
- Organize chains and links in separate files for clarity.
- This pattern is ideal for most workflows and onboarding new contributors.
---