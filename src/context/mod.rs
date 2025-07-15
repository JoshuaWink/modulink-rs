//! Context type for modulink-rust
//! Serializable, type-safe map for passing data between links.
//!
//! # Shadowing Policy (Ergonomic APIs)
//! For ergonomic usage (`Context`), always use variable shadowing (e.g., `let ctx = ctx.insert(...)`) instead of `mut`.
//! This prevents accidental mutation and is safer for async/concurrent code. See migration plan for details.
//!
//! Example (ergonomic, shadowing):
//! ```rust
//! use modulink_rs::context::Context;
//! let ctx = Context::new();
//! let ctx = ctx.insert("a", 1);
//! let ctx = ctx.insert("b", 2);
//! ```
//!
//! Advanced/generic APIs (ContextMutable) may use `mut` for performance, but must document the tradeoff.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Context(pub HashMap<String, Value>);

impl Context {
    pub fn new() -> Self {
        Context(HashMap::new())
    }
    // Immutable insert: returns a new Context with the value inserted
    pub fn insert<K: Into<String>, V: Serialize>(self, key: K, value: V) -> Self {
        let mut new_ctx = self;
        new_ctx.0.insert(key.into(), serde_json::to_value(value).unwrap());
        new_ctx
    }
    pub fn get<V: for<'de> Deserialize<'de>>(&self, key: &str) -> Option<V> {
        self.0.get(key).and_then(|v| serde_json::from_value(v.clone()).ok())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ContextMutable(pub HashMap<String, Value>);

impl ContextMutable {
    pub fn new() -> Self {
        ContextMutable(HashMap::new())
    }
    // Mutable insert: mutates in place
    pub fn insert<K: Into<String>, V: Serialize>(&mut self, key: K, value: V) {
        self.0.insert(key.into(), serde_json::to_value(value).unwrap());
    }
    pub fn get<V: for<'de> Deserialize<'de>>(&self, key: &str) -> Option<V> {
        self.0.get(key).and_then(|v| serde_json::from_value(v.clone()).ok())
    }
}
