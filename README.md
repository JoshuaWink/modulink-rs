# ModuLink-RS

Composable, observable, and testable async/sync chains for Rust. ModuLink-RS enables modular orchestration of pure functions with context-passing, middleware, and branching—designed for clarity, safety, and real-world scale.

## Quick Start: Ergonomic Chain Macro

The easiest way to compose async steps is with the `chain!` macro:

```rust
use modulink_rs::chain;
use modulink_rs::context::Context;

fn add_key_link(key: &'static str, value: i32) -> impl Fn(Context) -> std::pin::Pin<Box<dyn std::future::Future<Output = Context> + Send>> + Send + Sync {
    move |ctx: Context| Box::pin(async move {
        ctx.insert(key, value)
    })
}

#[tokio::main]
async fn main() {
    let chain = chain![
        add_key_link("a", 1),
        add_key_link("b", 2),
    ];
    let ctx = Context::new();
    let result = chain.run(ctx).await;
    assert_eq!(result.get::<i32>("a"), Some(1));
    assert_eq!(result.get::<i32>("b"), Some(2));
}
```

This pattern is recommended for most users. For advanced usage, see below.

## Shadowing Policy (Ergonomic APIs)
- For all ergonomic usage (`Chain`, `Link`, `Context`), always use variable shadowing (e.g., `let ctx = ...`) instead of `mut`.
- This prevents accidental mutation and is safer for async/concurrent code. See migration plan for details.
- Advanced/generic APIs may use `mut` for performance, but must document the tradeoff.

## Features
- **Ergonomic API:** Simple, safe, and easy to learn. Compose async/sync functions and handlers with minimal boilerplate.
- **Generic Power:** Use custom context types and advanced generics for maximum flexibility.
- **Context-Driven:** Type-safe, immutable or mutable context for data flow between links.
- **Middleware:** Plug in logging, timing, error handling, and more.
- **Branching:** Build conditional flows and complex pipelines.
- **Listeners:** Integrate with HTTP, CLI, or custom triggers.
- **Battle-Tested:** Robust, well-documented, and fully tested.

## Quick Example: Async and Sync Handler with Listener
```rust
use modulink_rs::context::Context;
use modulink_rs::links::{Handler, ListenerAsync, HttpListener};
use std::sync::Arc;
use std::future::Future;
use std::pin::Pin;

// Async handler
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

// Sync handler
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

// Listener usage
let handler = SyncEchoHandler::new();
let listener = HttpListener { handler, addr: "127.0.0.1:8089".to_string() };
tokio::spawn(async move { listener.start().await.unwrap(); });
// ...send requests to http://127.0.0.1:8089/run ...
```

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
- See the migration plan for rationale and details.

## Advanced Usage
For custom data models, mutable state, or domain-specific logic, use generics:
```rust
use modulink_rs::{chains::Chain, links::Link};

struct MyContext { /* ... */ }

fn my_link() -> Link<MyContext> {
    Box::new(|ctx: MyContext| Box::pin(async move {
        // ... mutate ctx ...
        ctx
    }))
}

let mut chain = Chain::<MyContext>::new();
chain.add_link(my_link());
// ...
```

## Documentation
- [User Guide](./docs/USER_GUIDE.md): Concepts, patterns, and best practices
- [Cheatsheet](./docs/CHEATSHEET.md): Quick reference for ergonomic usage
- [Advanced Cheatsheet](./docs/CHEATSHEET_ADVANCED.md): Power user and generic patterns
- [Simple Examples](./docs/SIMPLE_EXAMPLES.md): Copy-paste recipes

## Getting Started
1. Add ModuLink-RS to your `Cargo.toml`.
2. See the [User Guide](./docs/USER_GUIDE.md) for onboarding.
3. Explore the [Cheatsheet](./docs/CHEATSHEET.md) for quick syntax.
4. Dive into [Advanced Cheatsheet](./docs/CHEATSHEET_ADVANCED.md) for custom and generic workflows.

## License
Apache 2.0

---

For questions, see the docs or open an issue. May your code be clear and your chains unbroken.
