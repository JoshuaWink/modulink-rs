# Chain Macro: Ergonomic vs. Generic Usage

## Ergonomic (Default Context)

```rust
use modulink_rs::{chain, Context};

let chain = chain![
    link!(|ctx| async move { ctx.insert("foo", 1) }),
    link!(|ctx| async move { ctx.insert("bar", 2) }),
];
let ctx = Context::new();
let result = chain.run(ctx).await;
```

## Generic (Custom Context Type)

```rust
use modulink_rs::{chain, ContextMutable, LinkGeneric};

type MyContext = ContextMutable;

fn validate_input() -> LinkGeneric<MyContext> { /* ... */ }
fn transform_input() -> LinkGeneric<MyContext> { /* ... */ }
fn enrich_input() -> LinkGeneric<MyContext> { /* ... */ }

let chain = chain![type = MyContext;
    validate_input(),
    transform_input(),
    enrich_input(),
];
let ctx = MyContext::new().insert("input", "hello");
let result = chain.run(ctx).await;
```

**Rationale:**
- Use `chain![ ... ]` for ergonomic pipelines with the default `Context`.
- Use `chain![type = MyContext; ... ]` for generic/custom context types.
- The `type` parameter is optional for ergonomic usage, required for generics.

See the macro reference for more details and troubleshooting.
# ModuLink-RS Advanced Cheatsheet

## Ergonomic Usage (very brief)
- For most use cases, use `Context`, `Chain`, and `Link` as shown in the main cheatsheet.
- See `docs/CHEATSHEET.md` for ergonomic patterns.
---

## When to Use Advanced Patterns

Advanced usage is recommended when:
- You are chasing maximum performance and need fine-grained control over memory or concurrency.
- Your project has strict requirements for type safety, mutability, or integration with external systems.
- You are building for global scale, where custom context types or branching logic are essential.
- Your team is large and needs explicit contracts, extensibility, or domain-specific abstractions.

**Engineering Tax:**
- Advanced usage adds complexity and requires deeper Rust knowledge.
- There is a higher cognitive load for onboarding and maintenance.
- More boilerplate and stricter type signatures are required.

**Reward:**
- You gain maximum flexibility, type safety, and composability for complex or evolving systems.
- Your codebase becomes more robust, testable, and future-proof for large-scale or mission-critical applications.
- Enables domain-specific optimizations and integrations that are not possible with the ergonomic default.

---

# Advanced Usage: Full Guide

## 1. Generic Chains and Links
- Use generics to build chains and links with any context type: `Chain<T>`, `Link<T>`.
- Enables custom data models, mutable state, or integration with external systems.

### Example: Custom Context Type
```rust
use modulink_rs::chains::Chain;
use std::sync::Arc;
use std::future::Future;
use std::pin::Pin;

#[derive(Debug, Clone)]
struct MyContext {
    pub user_id: u64,
    pub data: String,
}

fn custom_link() -> Arc<dyn Fn(MyContext) -> Pin<Box<dyn Future<Output = MyContext> + Send>> + Send + Sync> {
    Arc::new(|ctx: MyContext| Box::pin(async move {
        MyContext { user_id: ctx.user_id, data: ctx.data + " processed" }
    }))
}

#[tokio::main]
async fn main() {
    let mut chain = Chain::<MyContext>::new();
    chain.add_link(custom_link());
    let ctx = MyContext { user_id: 42, data: "start".to_string() };
    let result = chain.run(ctx).await;
    println!("{:?}", result);
}
```


## 2. ContextMutable for In-Place Mutation
- Use `ContextMutable` for scenarios requiring in-place mutation or compatibility with APIs expecting mutability.
- Signature: `Chain<ContextMutable>`, `Link<ContextMutable>`.

### Example: ContextMutable
```rust
use modulink_rs::{ContextMutable, Chain};
use std::sync::Arc;
use std::future::Future;
use std::pin::Pin;

fn mut_link() -> Arc<dyn Fn(ContextMutable) -> Pin<Box<dyn Future<Output = ContextMutable> + Send>> + Send + Sync> {
    Arc::new(|mut ctx: ContextMutable| Box::pin(async move {
        ctx.insert("counter", 1);
        ctx
    }))
}

#[tokio::main]
async fn main() {
    let mut chain = Chain::<ContextMutable>::new();
    chain.add_link(mut_link());
    let ctx = ContextMutable::new();
    let result = chain.run(ctx).await;
    println!("{:?}", result.get::<i32>("counter"));
}
```

## 3. Advanced Branching and Control Flow
- Use `.connect(source, target, condition)` for custom graph topologies.
- `condition` is a closure: `Fn(&T) -> bool + Send + Sync`.

### Example: Conditional Branching
```rust
chain.connect(0, 2, |ctx| ctx.get::<bool>("skip").unwrap_or(false));
```

## 4. Custom Middleware
- Implement the `Middleware` trait for custom logging, metrics, or side effects.
- Attach with `.use_middleware()`.

### Example: Custom Middleware
```rust
use modulink_rs::middleware::Middleware;
use std::sync::Arc;
use async_trait::async_trait;

struct MyLogger;

#[async_trait]
impl Middleware for MyLogger {
    async fn before(&self, _ctx: &dyn std::any::Any) {
        println!("Before link");
    }
    async fn after(&self, _ctx: &dyn std::any::Any) {
        println!("After link");
    }
}

// Usage:
// chain.use_middleware(Arc::new(MyLogger));
```

## 5. Listeners and Integration
- Use listeners for HTTP, CLI, or custom triggers. Listeners support both async and sync handlers via the `Handler` trait.
- Implement your own or extend provided listeners for advanced orchestration.

### Example: Async and Sync Handler with Listener
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

## 6. Testing Advanced Chains
- Use generics in tests to validate custom context types, branching, and middleware.
- Example:
```rust
#[tokio::test]
async fn test_chain_with_custom_type() {
    let link = Arc::new(|ctx: MyContext| Box::pin(async move { MyContext { user_id: ctx.user_id, data: ctx.data + "!" } }));
    let mut chain = Chain::<MyContext>::new();
    chain.add_link(link);
    let ctx = MyContext { user_id: 1, data: "x".to_string() };
    let result = chain.run(ctx).await;
    assert_eq!(result.data, "x!".to_string());
}
```

## 7. Extending ModuLink-RS
- Build your own context types, middleware, or listeners for domain-specific needs.
- Use Rust's trait system and generics for maximum flexibility.

## 8. Reference: Key APIs
- `Chain<T>`: Generic chain, supports any context type.
- `Link<T>`: Generic async function, signature: `Fn(T) -> Pin<Box<dyn Future<Output = T> + Send>>`.
- `Middleware`: Trait for before/after hooks.
- `Listener`: For external triggers.
- `.connect()`: Add custom branches.
- `.use_middleware()`: Attach middleware.
- `.add_link()`: Add a link.
- `.run()`: Execute the chain.

## Shadowing Policy (Ergonomic APIs)
- For all ergonomic usage (`Chain`, `Link`, `Context`), always use variable shadowing (e.g., `let ctx = ...`) instead of `mut`.
- This prevents accidental mutation and is safer for async/concurrent code. See migration plan for details.
- Advanced/generic APIs may use `mut` for performance, but must document the tradeoff.

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

---

## Refresher (Advanced)
- context = any type (Context, MutableContext, or custom)
- link: Fn(T) -> T (async, generic)
- chain<T>(link1, link2, ...) -> newChain<T>
- chain.add_link(link<T>)
- chain.use_middleware(middleware)
- chain.connect(source, target, condition)
- chain.run(ctx: T)
- condition: Fn(&T) -> bool (for branching)
- middleware: before/after hooks, custom traits
- listener: for triggers (HTTP, CLI, etc.)
- maximize type safety, extensibility, domain-specific logic, extreme modularity
- this pattern allows for genuine test driven development