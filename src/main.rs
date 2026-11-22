use axum::{Json, Router, routing::get};
use serde_json::{Value, json};
use std::{
    env,
    sync::atomic::{AtomicU64, Ordering},
};

static COUNTER: AtomicU64 = AtomicU64::new(0);

async fn start() -> String {
    let visitors = COUNTER.fetch_add(1, Ordering::SeqCst) + 1;
    let host = env::var("HOSTNAME").unwrap_or_else(|_| "unkwnown".into());
    format!("Hello from {host}! You are visitor number {visitors}\n")
}

async fn health() -> Json<Value> {
    Json(json!({
        "status": "ok"
    }))
}

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "5000".into());
    let app = Router::new()
        .route("/", get(start))
        .route("/health", get(health));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{port}"))
        .await
        .expect("Could not open this port");
    println!("Listening on port {port}");
    axum::serve(listener, app).await.unwrap();
}
