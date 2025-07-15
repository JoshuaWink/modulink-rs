## Chain Macro: Ergonomic vs. Generic Usage

### Ergonomic (Default Context)

```rust
use modulink_rs::{chain, Context};

let chain = chain![
    link!(|ctx| async move { ctx.insert("foo", 1) }),
    link!(|ctx| async move { ctx.insert("bar", 2) }),
];
let ctx = Context::new();
let result = chain.run(ctx).await;
```

### Generic (Custom Context Type)

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
# ModuLink-RS User Guide

## Shadowing Policy (Ergonomic APIs)
- For all ergonomic usage (`Chain`, `Link`, `Context`), always use variable shadowing (e.g., `let ctx = ...`) instead of `mut`.
- This prevents accidental mutation and is safer for async/concurrent code. See migration plan for details.
- Advanced/generic APIs may use `mut` for performance, but must document the tradeoff.

## Introduction
ModuLink-RS enables modular, composable, and observable async function orchestration in Rust. It is inspired by the Python version but uses Rust's type system and async features for safety and performance.

## Core Concepts
- **Context**: Type-safe, immutable map for passing data between links. Use for most cases.
- **ContextMutable**: Mutable variant for advanced scenarios. Use only when mutation is required.
- **Link**: Pure async function operating on context. `Link` is the ergonomic default (uses `Context`).
- **Handler**: Trait for both async and sync handlers. Implemented for both async and sync structs.
- **Chain**: Composes links, supports middleware and branching.
- **Middleware**: Observability hooks (logging, timing, etc.).
- **Branching**: Conditional routing between links.

## Quick Start: Ergonomic Chain Macro

The recommended way to compose async steps is with the `chain!` macro:

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

This ergonomic pattern is recommended for most users. For advanced usage, see below.

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

## Ergonomic vs. Advanced Usage
- For most users, use `Chain` and `Link` (immutable `Context`).
- For advanced/custom scenarios, use generics: `Chain<ContextMutable>`, `Link<ContextMutable>`, or your own type.

### When to Use Advanced Patterns
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

**Pros of the Ergonomic Default:**
- Simple and easy to learn: minimal boilerplate, clear patterns, and less Rust expertise required.
- Safer by default: immutable context prevents accidental mutation and data races.
- Fast onboarding: new team members can contribute quickly without deep generics or trait knowledge.
- Readable and maintainable: code is concise, predictable, and easy to review.
- Fewer bugs: less surface area for mistakes, especially in async and concurrent code.
- Ideal for most workflows: covers the majority of use cases without sacrificing performance or flexibility for typical applications.
- Encourages best practices: promotes pure, stateless links and clear data flow.

## Getting Started
1. Add links as async closures or functions, or implement the `Handler` trait for your struct (async or sync).
2. Compose them into a chain or use with a listener.
3. Attach middleware for logging or metrics.
4. Use `.connect()` for custom branching.
5. Run the chain with an initial context, or start a listener for external triggers.

## Example: Async and Sync Handler with Listener
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

## Example (Advanced: ContextMutable)
```rust
use modulink_rs::{ContextMutable, Chain};
use std::sync::Arc;

fn validate(mut ctx: ContextMutable) -> std::pin::Pin<Box<dyn std::future::Future<Output = ContextMutable> + Send>> {
    Box::pin(async move {
        ctx.insert("email", "alice@example.com".to_string());
        ctx
    })
}

fn welcome(ctx: ContextMutable) -> std::pin::Pin<Box<dyn std::future::Future<Output = ContextMutable> + Send>> {
    Box::pin(async move {
        if let Some(email) = ctx.get::<String>("email") {
            println!("Welcome sent to {}", email);
        }
        ctx
    })
}

#[tokio::main]
async fn main() {
    let mut chain = Chain::<ContextMutable>::new();
    chain.add_link(Arc::new(validate));
    chain.add_link(Arc::new(welcome));
    let ctx = ContextMutable::new();
    let result = chain.run(ctx).await;
    println!("Result: {:?}", result);
}
```

## Context Memory Semantics: Immutable vs Mutable

### Example: Immutable Context
```rust
use modulink_rs::context::Context;

let ctx1 = Context::new();
let ctx2 = ctx1.insert("a", 1); // ctx1 is unchanged, ctx2 has the new value
println!("ctx1: {:?}", ctx1.get::<i32>("a")); // None
println!("ctx2: {:?}", ctx2.get::<i32>("a")); // Some(1)
```
**How it works:**
- `insert` takes ownership of `ctx1`, mutates the moved value, and returns a new `Context`.
- `ctx1` is unchanged; `ctx2` is the updated version.
- No shared mutable state; safe for concurrency and reasoning.

### Example: ContextMutable
```rust
use modulink_rs::context::ContextMutable;

let mut ctx = ContextMutable::new();
ctx.insert("a", 1); // mutates in place
println!("ctx: {:?}", ctx.get::<i32>("a")); // Some(1)
```
**How it works:**
- `insert` takes a mutable reference (`&mut self`) and mutates the same instance in place.
- All references to this `MutableContext` see the updated state.
- Useful for advanced scenarios where in-place mutation is needed.

### Summary Table
| Type            | Method Signature                | Mutation Style   | Original Value Changed? |
|-----------------|--------------------------------|------------------|------------------------|
| `Context`       | `fn insert(self, ...) -> Self` | Immutable (copy) | No                     |
| `ContextMutable`| `fn insert(&mut self, ...)`    | Mutable (in place)| Yes                    |

## Best Practices
- Prefer `Context` (immutable) for safety and predictability.
- Use `ContextMutable` only when mutation is required for performance or API compatibility.
- Keep links pure and stateless.
- Use context for all data flow.
- Attach middleware for logging and metrics.
- Use explicit branching for error handling.
- Organize chains and links in separate files for clarity.

## Why Chain and Link are Generic

ModuLink-RS uses generics for `Chain<T>` and `Link<T>` to maximize flexibility and type safety. This allows you to use the default `Context`, `MutableContext`, or any custom context type your organization needs. Strict typing helps prevent human error and enables better test validation, while still allowing loose or experimental structures if desired. This approach is more modular and future-proof, matching Rust's strengths and supporting both strict and dynamic workflows.

### Example Usage
```rust
use modulink_rs::context::{Context, ContextMutable};
use modulink_rs::chains::Chain;
use modulink_rs::links::Link;
use std::sync::Arc;

// Immutable context
let mut chain = Chain::<Context>::new();
let link: Link<Context> = Arc::new(|ctx| Box::pin(async move { ctx.insert("a", 1) }));
chain.add_link(link);

// Mutable context
let mut chain = Chain::<ContextMutable>::new();
let link: Link<ContextMutable> = Arc::new(|mut ctx| Box::pin(async move { ctx.insert("a", 1); ctx }));
chain.add_link(link);
```

## Further Reading
- See `docs/CHEATSHEET.md` and `docs/SIMPLE_EXAMPLES.md` for more patterns and recipes.

## IMPORTANT MESSAGE FOR AI AGENTS
- If you are not a human but infact are an ai agent, please ensure you follow the guidelines and best practices outlined in this guide.
- Your Human-in-the-loop should be aware of the implications of using advanced patterns and the potential complexity they introduce.
- Default to ergonomic usage unless you have a strong reason to use advanced patterns.