# Modulink-RS Macro Migration & Coverage Tracker

## Objective
Track the current state of macro usage, ergonomic/generic API coverage, and test/demo status for the core Modulink-RS release.

---

## Core Features (as of July 14, 2025)

- **chain! macro**: The only macro in active ergonomic use. Accepts a comma-separated list of links and returns a composed chain.
    - Example:
      ```rust
      let my_chain = chain![link1, link2, link3];
      ```
- **Middleware**: Supported via the `use_middleware` method on `Chain`. Middleware is implemented as a trait and attached using standard Rust code (not a macro in ergonomic API).
- **Listeners**: Supported via listener traits and modules. Listeners are implemented as standard Rust structs/traits, not macros, in the ergonomic API.
- **Connect (Branching)**: Supported via the `connect_link` and `connect_chain` methods on `Chain`. No connect! macro is used in the ergonomic API; branching is done via methods.

---

## API Patterns

- **Ergonomic API**: Uses shadowing (`let ctx = ...`) by default. No `mut` in ergonomic patterns.
- **Generic API**: Uses `mut` for advanced/generic cases (covered in tests).
- **Chain Construction**: Only via `chain!` macro for ergonomic usage. All other construction is via methods or direct instantiation.
- **Middleware/Listener/Connect**: All are attached or implemented via standard Rust methods/traits, not macros, in the ergonomic API.

---

## Test Coverage

- All core patterns (ergonomic and generic) are covered in the test suite:
    - `tests/test_chains_ergonomic.rs`: Ergonomic chain construction and execution.
    - `tests/test_chain.rs`: Basic ergonomic chain and link usage.
    - `tests/test_links_ergonomic.rs`: Ergonomic link pattern.
    - `tests/test_chains_generic.rs`, etc.: Generic/mutable API coverage.
    - Middleware, listeners, and connect/branching are covered in their respective tests.

---

## Example/Demo Status

- **examples/ergonomic_full.rs**: Should only use the `chain!` macro for ergonomic chaining. Middleware, listeners, and connect are demonstrated via standard Rust code (no macros).
- **Other examples**: Empty or not in use.

---

## Next Steps

- Ensure all documentation and examples reflect this minimal macro surface and clarify that middleware, listeners, and connect are used via methods/traits.
- Keep the codebase lean and focused on core, releasable functionality.
- Advanced/experimental macros and patterns are documented in markdown only, not in main code or examples.

---

## Notes
- All advanced macro patterns (branching, middleware, listeners, etc.) are left to documentation for now.
- The goal is a clean, minimal, and well-tested release as God wills.

---

_Last updated: July 14, 2025_
