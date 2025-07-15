# Modulink-RS Chain Macro Patterns

This document outlines macro patterns for ergonomic and advanced (mutating) chain composition in Rust, inspired by the `modulink-py` cheatsheet and core concepts.

## Core Concepts (from modulink-py)
- **Context**: Data passed through all links in a chain.
- **Link**: Pure function (or async function in Python).
- **Chain**: Sequence of links, executed in order.
- **Middleware**: Optional, for observability/logging.
- **Branching**: Conditional flows between links.

---

## Ergonomic (Shadowing) Chain Macro

- **Pattern:** Each link receives context, returns new context. No mutation; safe for async/concurrent use.
- **Macro:** `chain![link1, link2, link3]`
- **Usage:**

```rust
let my_chain = chain![Link1, Link2, Link3];
let result = my_chain.run(initial_context);
```

- **Macro Draft:**
```rust
macro_rules! chain {
    [ $($link:expr),+ $(,)? ] => {
        {
            let links: Vec<Box<dyn Link>> = vec![$(Box::new($link)),+];
            Chain { links }
        }
    };
}
```

- **Trait and Struct Example:**
```rust
trait Link {
    fn call(&self, ctx: Context) -> Context;
}

struct Chain {
    links: Vec<Box<dyn Link>>,
}

impl Chain {
    fn run(&self, mut ctx: Context) -> Context {
        for link in &self.links {
            ctx = link.call(ctx);
        }
        ctx
    }
}
```

---

## Advanced (Mutating) Chain Macro

- **Pattern:** Each link mutates context in place. Allowed only for advanced/generic APIs; must document tradeoffs.
- **Macro:** `mut_chain![link1, link2, link3]`
- **Usage:**

```rust
let mut my_chain = mut_chain![Link1, Link2, Link3];
my_chain.run_in_place(&mut initial_context);
```

- **Macro Draft:**
```rust
macro_rules! mut_chain {
    [ $($link:expr),+ $(,)? ] => {
        {
            let links: Vec<Box<dyn MutLink>> = vec![$(Box::new($link)),+];
            MutableChain { links }
        }
    };
}
```

- **Trait and Struct Example:**
```rust
trait MutLink {
    fn call(&self, ctx: &mut Context);
}

struct MutableChain {
    links: Vec<Box<dyn MutLink>>,
}

impl MutableChain {
    fn run_in_place(&self, ctx: &mut Context) {
        for link in &self.links {
            link.call(ctx);
        }
    }
}
```

---

## Design Rationale
- **Ergonomic API:**
    - Uses shadowing (`let ctx = ...`) for safety and composability.
    - Prevents accidental mutation and race conditions.
    - Matches Python's pure function chain style.
- **Advanced API:**
    - Allows mutation for performance or special cases.
    - Requires explicit opt-in via macro name (`mut_chain!`).
    - Must document risks and tradeoffs.

---

## Example: Side-by-Side

```rust
// Ergonomic (shadowing)
let ctx = chain![A, B, C].run(ctx);

// Advanced (mutating)
let mut ctx = ...;
mut_chain![A, B, C].run_in_place(&mut ctx);
```

---

## Next Steps
- Integrate these macro patterns into the codebase.
- Update documentation and onboarding to reflect both styles.
- Encourage ergonomic usage by default; reserve mutation for advanced needs, as God wills.
