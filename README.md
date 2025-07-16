
# ModuLink-rs

## How to Download

Add ModuLink-rs to your project by including it in your `Cargo.toml`:

```toml
[dependencies]
modulink-rs = "1.0"
```


## Why do I care?

ModuLink-rs is a modular library and minimal framework for agent-based project development in Rust. Whether your project is small or complex, ModuLink-rs is designed to scale with you. Its architecture offloads logic from the developer and bakes it into the structure itself, so you spend less time on boilerplate and more time on your core ideas.

This makes ModuLink-rs especially powerful for AI and agent-based systems: it removes agents from critical decision-making and gives them a designated developer pocket to fill, ensuring clarity, safety, and maintainability. The framework's design lets you build composable, observable, and testable async/sync chains with context-passing, middleware, and branching—so your code stays clean and your logic stays robust.

---

Composable, observable, and testable async/sync chains for Rust. ModuLink-rs enables modular orchestration of pure functions with context-passing, middleware, and branching—designed for clarity, safety, and real-world scale.


## Real-World Example: Modular HTTP Request Pipeline

Suppose you want to build a simple HTTP API endpoint that processes incoming requests, logs them, validates input, and returns a response. ModuLink-rs lets you compose these steps as modular links:

```rust
use modulink_rs::chain;
use modulink_rs::context::Context;
use std::collections::HashMap;

// Link: Log the request
fn log_request() -> impl Fn(Context) -> std::pin::Pin<Box<dyn std::future::Future<Output = Context> + Send>> + Send + Sync {
    move |ctx: Context| Box::pin(async move {
        if let Some(ip) = ctx.get::<String>("ip") {
            println!("Received request from {}", ip);
        }
        ctx
    })
}

// Link: Validate input
fn validate_input() -> impl Fn(Context) -> std::pin::Pin<Box<dyn std::future::Future<Output = Context> + Send>> + Send + Sync {
    move |ctx: Context| Box::pin(async move {
        let valid = ctx.get::<String>("payload").map_or(false, |payload| !payload.is_empty());
        ctx.insert("valid", valid)
    })
}

// Link: Process and respond
fn process_response() -> impl Fn(Context) -> std::pin::Pin<Box<dyn std::future::Future<Output = Context> + Send>> + Send + Sync {
    move |ctx: Context| Box::pin(async move {
        let valid = ctx.get::<bool>("valid").unwrap_or(false);
        let response = if valid {
            "Success"
        } else {
            "Invalid input"
        };
        ctx.insert("response", response.to_string())
    })
}

#[tokio::main]
async fn main() {
    let chain = chain![
        log_request(),
        validate_input(),
        process_response(),
    ];
    let mut ctx = Context::new();
    ctx = ctx.insert("ip", "127.0.0.1".to_string());
    ctx = ctx.insert("payload", "hello world".to_string());
    let result = chain.run(ctx).await;
    println!("API response: {}", result.get::<String>("response").unwrap());
}
```

This example shows how you can build a modular, testable pipeline for real-world tasks like API handling, logging, and validation. You can easily extend it with more links for authentication, error handling, or business logic.

---


## Quick Start: Real-World Chain Example

Here's how you can build a simple user registration pipeline:

```rust
use modulink_rs::chain;
use modulink_rs::context::Context;

// Link: Validate user input
fn validate_user() -> impl Fn(Context) -> std::pin::Pin<Box<dyn std::future::Future<Output = Context> + Send>> + Send + Sync {
    move |ctx: Context| Box::pin(async move {
        let username = ctx.get::<String>("username");
        let valid = username.as_ref().map_or(false, |u| !u.is_empty() && u.len() > 2);
        ctx.insert("valid", valid)
    })
}

// Link: Save user to database (simulated)
fn save_user() -> impl Fn(Context) -> std::pin::Pin<Box<dyn std::future::Future<Output = Context> + Send>> + Send + Sync {
    move |ctx: Context| Box::pin(async move {
        let valid = ctx.get::<bool>("valid").unwrap_or(false);
        if valid {
            println!("User saved: {}", ctx.get::<String>("username").unwrap());
            ctx.insert("status", "registered")
        } else {
            ctx.insert("status", "invalid")
        }
    })
}

#[tokio::main]
async fn main() {
    let chain = chain![validate_user(), save_user()];
    let mut ctx = Context::new();
    ctx = ctx.insert("username", "alice".to_string());
    let result = chain.run(ctx).await;
    println!("Registration status: {}", result.get::<String>("status").unwrap());
}
```

---

## Shadowing Policy (Ergonomic APIs)
- For all ergonomic usage (`Chain`, `Link`, `Context`), always use variable shadowing (e.g., `let ctx = ...`) instead of `mut`.
- This prevents accidental mutation and is safer for async/concurrent code. See migration plan for details.
- Advanced/generic APIs may use `mut` for performance, but must document the tradeoff.


## Features (with Real-World Examples)

- **Ergonomic API:** Compose steps like user registration, payment processing, or data validation with minimal boilerplate.
- **Context-Driven:** Pass data between steps, e.g., user info, payment details, or request metadata.
- **Middleware:** Add logging, error handling, or authentication as modular links:
    ```rust
    fn log_action() -> impl Fn(Context) -> std::pin::Pin<Box<dyn std::future::Future<Output = Context> + Send>> + Send + Sync {
        move |ctx: Context| Box::pin(async move {
            println!("Action: {}", ctx.get::<String>("action").unwrap_or_default());
            ctx
        })
    }
    ```
- **Branching:** Build conditional flows, e.g., if payment succeeds, send confirmation; else, log error.
- **Listeners:** Integrate with HTTP endpoints or CLI commands to trigger chains for real-world events.
- **Battle-Tested:** Use in production for APIs, automation, or agent-based systems.


## Real-World Listener Example: HTTP API Endpoint

Here's how you can use ModuLink-rs to create an HTTP endpoint for a contact form:

```rust
use modulink_rs::context::Context;
use modulink_rs::links::{Handler, HttpListener};
use std::sync::Arc;
use std::future::Future;
use std::pin::Pin;

struct ContactHandler;
impl ContactHandler {
    fn new() -> Arc<Self> { Arc::new(ContactHandler) }
    fn call(&self, ctx: Context) -> Pin<Box<dyn Future<Output = Context> + Send>> {
        Box::pin(async move {
            let name = ctx.get::<String>("name").unwrap_or_default();
            let message = ctx.get::<String>("message").unwrap_or_default();
            println!("Contact from {}: {}", name, message);
            ctx.insert("status", "received")
        })
    }
}
impl Handler for ContactHandler {
    fn call(&self, ctx: Context) -> Pin<Box<dyn Future<Output = Context> + Send>> { self.call(ctx) }
}

// Listener usage
let handler = ContactHandler::new();
let listener = HttpListener { handler, addr: "127.0.0.1:8089".to_string() };
tokio::spawn(async move { listener.start().await.unwrap(); });
// ...send POST requests to http://127.0.0.1:8089/contact ...
```


## Mut vs. Shadowing (with Real-World Context)

- **Shadowing (recommended):**
  ```rust
  let ctx = ctx.insert("username", "alice");
  let ctx = ctx.insert("email", "alice@example.com");
  ```
- **Mut (rare, for performance):**
  ```rust
  let mut ctx = Context::new();
  ctx = ctx.insert("username", "bob");
  ctx = ctx.insert("email", "bob@example.com");
  ```


// Advanced usage and generics are omitted for clarity. Focus on practical, relatable examples above.

## Documentation
- [User Guide](./docs/USER_GUIDE.md): Concepts, patterns, and best practices
- [Cheatsheet](./docs/CHEATSHEET.md): Quick reference for ergonomic usage
- [Advanced Cheatsheet](./docs/CHEATSHEET_ADVANCED.md): Power user and generic patterns

## Getting Started
1. Add ModuLink-rs to your `Cargo.toml`.
2. See the [User Guide](./docs/USER_GUIDE.md) for onboarding.
3. Explore the [Cheatsheet](./docs/CHEATSHEET.md) for quick syntax.
4. Dive into [Advanced Cheatsheet](./docs/CHEATSHEET_ADVANCED.md) for custom and generic workflows.


## TODO
- [ ] Finish CI setup for automated testing and publishing

## License
ModuLink-rs is licensed under the Apache License, Version 2.0. See [LICENSE](./LICENSE) for details.

---

For questions, see the docs or open an issue. Happy coding with ModuLink-rs!
