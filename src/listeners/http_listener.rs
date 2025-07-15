



use axum::{Router, routing::post, extract::State, Json};
use crate::context::Context;
use crate::listeners::BaseListenerAsync;
use std::sync::Arc;
use std::net::SocketAddr;
use async_trait::async_trait;



/// Default ergonomic HTTP listener for modulink-rust using axum.
/// Accepts a handler (chain) and address.
pub struct HttpListener {
    pub handler: Arc<dyn Fn(Context) -> std::pin::Pin<Box<dyn std::future::Future<Output = Context> + Send>> + Send + Sync>,
    pub addr: String,
}




#[async_trait]
impl BaseListenerAsync for HttpListener {
    async fn start(&self) -> std::io::Result<()> {
        let handler = self.handler.clone();
        let addr: SocketAddr = self.addr.parse().expect("Invalid address");

        // Axum handler closure
        let handler_clone = handler.clone();
        let app = Router::new()
            .route("/run", post(
                move |State(handler): State<Arc<dyn Fn(Context) -> std::pin::Pin<Box<dyn std::future::Future<Output = Context> + Send>> + Send + Sync>>, Json(body): Json<serde_json::Value>| {
                    let handler = handler.clone();
                    async move {
                        let map = body.as_object().cloned().unwrap_or_default();
                        let ctx = Context(map.into_iter().collect());
                        let result = handler(ctx).await;
                        // Convert HashMap to serde_json::Map for correct JSON response
                        let map: serde_json::Map<String, serde_json::Value> = result.0.into_iter().collect();
                        Json(serde_json::Value::Object(map))
                    }
                }
            ))
            .with_state(handler_clone);

        // Use axum::serve (hyper::Server)
        use axum::serve;
        use tokio::net::TcpListener;
        let listener = TcpListener::bind(addr).await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        serve(listener, app.into_make_service()).await.map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
    }
    fn name(&self) -> &'static str {
        "http"
    }
}
