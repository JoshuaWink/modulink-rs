# Modulink-RS Macro Reference

This document provides well-annotated, idiomatic Rust examples for all macros and patterns that simplify the modulink-rust workflow. The structure and clarity are inspired by modulink-py, but all code is idiomatic Rust. Each section includes macro definition, usage, and rationale.

---

## 1. Context Macro

**Purpose:** Define or initialize a context (can be `Context`, `MutableContext`, or custom type).

```rust
// Example: Using the default Context type
define_context! {
    user_id: u64,
    data: String,
}

// Or simply:
let ctx = Context::new();
```

*Rationale:* Context is the data passed through all links. Use custom types for domain-specific needs.

---

## 2. Link Macro

**Purpose:** Define a link as a pure async function or closure.

```rust
link! {
    fn add_user_id(ctx: Context) -> Context {
        ctx.insert("user_id", 42)
    }
}

// Or as a closure:
let link = link!(|ctx: Context| async move { ctx.insert("user_id", 42) });
```

*Rationale:* Links are pure, composable steps. Use async for IO or concurrency.

---

## 3. Chain Macro

**Purpose:** Compose a sequence of links into a chain.

```rust
let my_chain = chain![add_user_id, another_link, final_link];
```

*Rationale:* Chains are pipelines. The macro makes composition concise and readable.

---

## 4. Add Link Macro

**Purpose:** Add a link to an existing chain.

```rust
my_chain.add_link(link!(|ctx| async move { ctx }));
```

*Rationale:* Enables dynamic or incremental chain construction.

---

## 5. Use Middleware Macro

**Purpose:** Attach middleware for logging, metrics, or side effects.

```rust
my_chain.use_middleware(middleware!(Logging));
```

*Rationale:* Middleware provides observability and cross-cutting concerns.

---

## 6. Connect Macro (Branching)

**Purpose:** Add conditional branches between links or chains, using a macro syntax that clarifies intent and supports both link and chain connections.

### Macro Syntax Options

#### Connect a Link
```rust
my_chain.connect[
    link: my_link,
    to: link_in_og_chain,
    when: condition!(|ctx| ctx.get::<bool>("skip").unwrap_or(false)),
]
```

#### Connect a Chain
```rust
my_chain.connect[
    chain: my_other_chain,
    to: link_in_og_chain,
    when: condition!(|ctx| ctx.get::<bool>("should_branch").unwrap_or(false)),
]
```

- `link`: The new link to connect (mutually exclusive with `chain`).
- `chain`: The new chain to connect (mutually exclusive with `link`).
- `to`: The target link in the original chain to branch to.
- `when`: A closure that takes a reference to the context and returns a bool.

*Rationale:* This macro syntax makes branching explicit and readable, supporting both single-link and sub-chain connections. It enables advanced graph topologies, error routing, and dynamic control flow, while making intent clear and reducing ambiguity.

**Note:** Only one of `link` or `chain` should be provided per connect statement.

---

## 7. Run Macro

**Purpose:** Execute the chain with a given context.

```rust
let result = my_chain.run(ctx).await;
```

*Rationale:* Runs the pipeline. Async for concurrency.

---

## 8. Condition Macro

**Purpose:** Define a branching condition as a closure.

```rust
let cond = condition!(|ctx: &Context| ctx.get::<bool>("flag").unwrap_or(false));
```

*Rationale:* Used for branching and control flow.

---

## 9. Middleware Macro

**Purpose:** Define custom middleware with before/after hooks.

```rust
middleware! {
    struct MyLogger;
    impl Middleware for MyLogger {
        async fn before(&self, ctx: &dyn std::any::Any) {
            println!("Before link");
        }
        async fn after(&self, ctx: &dyn std::any::Any) {
            println!("After link");
        }
    }
}
```

*Rationale:* Custom observability, metrics, or side effects.

---

## 10. Listener Macro

**Purpose:** Define a listener for triggers (HTTP, CLI, etc.).

```rust
listener! {
    struct MyHttpListener;
    // ... implement listener trait ...
}
```

*Rationale:* Integrates chains with external systems.

---

## Notes
- All macros are sketches; adapt as needed for your codebase.
- Use shadowing (`let ctx = ...`) for ergonomic APIs; use `mut` only for advanced/generic APIs and document the tradeoff.
- Maximize type safety, extensibility, and modularity as God wills.

---

## Example: Full Pipeline

```rust
let ctx = Context::new();
let chain = chain![
    link!(|ctx| async move { ctx.insert("step", 1) }),
    link!(|ctx| async move { ctx.insert("step", ctx.get::<i32>("step").unwrap() + 1) })
];
let result = chain.run(ctx).await;
```

---

## Further Reading
- See `docs/CHAIN_MACRO_PATTERNS.md` for macro design rationale.
- See `docs/CHEATSHEET_ADVANCED.md` for advanced usage patterns.
