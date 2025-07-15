//! Chain struct for composing and running links with context, middleware, and branching.
//!
//! # Shadowing Policy (Ergonomic APIs)
//! For ergonomic usage (`Chain`, `Link`, `Context`), always use variable shadowing (e.g., `let ctx = ...`) instead of `mut`.
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
//! Advanced/generic APIs may use `mut` for performance, but must document the tradeoff.

use std::sync::Arc;
use std::future::Future;
use std::pin::Pin;

// Generic Chain: works with any context type (Context, MutableContext, or user-defined)
pub struct ChainGeneric<T> {
    links: Vec<Arc<dyn Fn(T) -> Pin<Box<dyn Future<Output = T> + Send>> + Send + Sync>>,
    middleware: Vec<Arc<dyn crate::middleware::Middleware<T>>>,
    pub branches: Vec<Branch<T>>,
}

pub struct Branch<T> {
    pub source: usize,
    pub target: usize,
    pub condition: Arc<dyn Fn(&T) -> bool + Send + Sync>,
}

impl<T: 'static + Send> ChainGeneric<T> {
    pub fn new() -> Self {
        ChainGeneric { links: Vec::new(), middleware: Vec::new(), branches: Vec::new() }
    }
    pub fn add_link(&mut self, link: Arc<dyn Fn(T) -> Pin<Box<dyn Future<Output = T> + Send>> + Send + Sync>) {
        self.links.push(link);
    }
    pub fn use_middleware(&mut self, mw: Arc<dyn crate::middleware::Middleware<T>>) {
        self.middleware.push(mw);
    }
    pub fn link_count(&self) -> usize {
        self.links.len()
    }
    pub fn connect<F>(&mut self, source: usize, target: usize, condition: F)
    where
        F: Fn(&T) -> bool + Send + Sync + 'static,
    {
        self.branches.push(Branch {
            source,
            target,
            condition: Arc::new(condition),
        });
    }
    pub async fn run(&self, ctx: T) -> T {
        let mut idx = 0;
        let mut ctx = ctx;
        while idx < self.links.len() {
            for mw in &self.middleware {
                mw.before(&ctx).await;
            }
            ctx = (self.links[idx].clone())(ctx).await;
            for mw in &self.middleware {
                mw.after(&ctx).await;
            }
            // Check for branch
            if let Some(branch) = self.branches.iter().find(|b| b.source == idx && (b.condition)(&ctx)) {
                idx = branch.target;
            } else {
                idx += 1;
            }
        }
        ctx
    }
}

// Ergonomic defaults
pub type Chain = ChainGeneric<crate::context::Context>;
pub type LinkGeneric<C> = crate::links::LinkGeneric<C>;
pub type Link = crate::links::Link;

// Optionally, re-export as Chain/Link for crate root (see lib.rs)
// pub use Chain as Chain;
// pub use Link as Link;
