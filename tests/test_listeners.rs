//! Integration test for the new ergonomic async listener API.
//
// This test follows the API and requirements in docs/LISTENER_API_DESIGN.md.

use modulink_rs::context::Context;
use modulink_rs::links::ListenerAsync;
use std::sync::Arc;
use std::future::Future;
use std::pin::Pin;

// Example async handler struct
struct EchoHandler;

impl EchoHandler {
    fn new() -> Arc<Self> {
        Arc::new(EchoHandler)
    }
}

impl EchoHandler {
    fn call(&self, ctx: Context) -> Pin<Box<dyn Future<Output = Context> + Send>> {
        Box::pin(async move {
            let val: Option<String> = ctx.get("input");
            ctx.insert("output", val.unwrap_or_else(|| "none".to_string()))
        })
    }
}

// Example sync handler struct
struct SyncEchoHandler;

impl SyncEchoHandler {
    fn new() -> Arc<Self> {
        Arc::new(SyncEchoHandler)
    }
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


use axum::{Router, routing::post, extract::State, Json, serve};
use tokio::net::TcpListener;
use serde_json::Value;
use async_trait::async_trait;
use std::net::SocketAddr;
use tokio::time::{sleep, Duration};

// Handler trait for async listeners
pub trait Handler: Send + Sync + 'static {
    fn call(&self, ctx: Context) -> Pin<Box<dyn Future<Output = Context> + Send>>;
}

impl Handler for EchoHandler {
    fn call(&self, ctx: Context) -> Pin<Box<dyn Future<Output = Context> + Send>> {
        self.call(ctx)
    }
}

struct HttpListener<H: Handler> {
    handler: Arc<H>,
    addr: String,
}

#[async_trait]
impl<H: Handler> ListenerAsync for HttpListener<H> {
    async fn start(&self) -> std::io::Result<()> {
        let handler = self.handler.clone();
        let addr: SocketAddr = self.addr.parse().expect("Invalid address");

        async fn run_handler<H: Handler>(
            State(handler): State<Arc<H>>,
            Json(body): Json<Value>,
        ) -> Json<Value> {
            let map = body.as_object().cloned().unwrap_or_default();
            let ctx = Context(map.into_iter().collect());
            let result = handler.call(ctx).await;
            let map: serde_json::Map<String, Value> = result.0.into_iter().collect();
            Json(Value::Object(map))
        }

        let app = Router::new()
            .route("/run", post(run_handler::<H>))
            .with_state(handler);

        let listener = TcpListener::bind(addr).await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        serve(listener, app.into_make_service()).await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }
    fn name(&self) -> &'static str {
        "http"
    }
}

#[tokio::test]
async fn test_http_listener_integration_pattern() {
    // Construct handler and listener
    let handler = EchoHandler::new();
    let listener = HttpListener { handler, addr: "127.0.0.1:8088".to_string() };
    // Start the listener in the background
    let server = tokio::spawn(async move {
        listener.start().await.unwrap();
    });
    // Wait for server to start
    sleep(Duration::from_millis(300)).await;
    // Send a POST request
    let client = reqwest::Client::new();
    let resp = client.post("http://127.0.0.1:8088/run")
        .json(&serde_json::json!({"input": "hello"}))
        .send().await.unwrap();
    let json: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(json["output"], "hello");
    drop(server);
}

#[tokio::test]
async fn test_http_listener_sync_handler() {
    // Construct sync handler and listener
    let handler = SyncEchoHandler::new();
    let listener = HttpListener { handler, addr: "127.0.0.1:8089".to_string() };
    // Start the listener in the background
    let server = tokio::spawn(async move {
        listener.start().await.unwrap();
    });
    // Wait for server to start
    sleep(Duration::from_millis(300)).await;
    // Send a POST request
    let client = reqwest::Client::new();
    let resp = client.post("http://127.0.0.1:8089/run")
        .json(&serde_json::json!({"input": "sync-test"}))
        .send().await.unwrap();
    let json: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(json["output"], "sync-test");
    drop(server);
}
