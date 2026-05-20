//! Zenvu.js Dev Server — HTTP server with WebSocket HMR.

pub mod hmr;
pub mod ssr;
pub mod ssg;
pub mod edge;
pub mod isr;
pub mod server_actions;
pub mod router;
pub mod orm;
pub mod queue;
pub mod mail;
pub mod storage;
pub mod streaming;
pub mod server_component;

use anyhow::Result;
use axum::{Router, routing::get, response::Html, response::Json};
use tower_http::services::ServeDir;
use std::net::SocketAddr;
use serde_json::{json, Value};

/// Start the development server with HMR and dynamic API endpoints support.
pub async fn start_dev_server(addr: &str) -> Result<()> {
    let api_routes = router::ApiRouter::new()
        .get("/health", health_handler)
        .get("/users", get_users_handler)
        .post("/users", create_user_handler);

    let app = Router::new()
        .route("/", get(index_handler))
        .route("/__zenvu_hmr", get(hmr::ws_handler))
        .nest("/api", api_routes.into_router())
        .fallback_service(ServeDir::new("src"));

    let addr: SocketAddr = addr.parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("Dev server listening on {}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}

/// Start a static preview server.
pub async fn start_preview_server(addr: &str, dir: &str) -> Result<()> {
    let app = Router::new()
        .fallback_service(ServeDir::new(dir));

    let addr: SocketAddr = addr.parse()?;
    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("Preview server listening on {}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn index_handler() -> Html<String> {
    let html = std::fs::read_to_string("index.html")
        .unwrap_or_else(|_| {
            r#"<!DOCTYPE html><html><body>
            <div id="app"></div>
            <script type="module" src="/main.ts"></script>
            <script type="module" src="/__zenvu_hmr_client.js"></script>
            </body></html>"#.to_string()
        });
    Html(html)
}

// ==========================================
// Zenvu Server API handlers
// ==========================================

async fn health_handler() -> Json<Value> {
    Json(json!({
        "success": true,
        "status": "healthy",
        "framework": "Zenvu.js Core Server Engine"
    }))
}

async fn get_users_handler() -> Json<Value> {
    Json(json!({
        "success": true,
        "users": [
            { "id": 1, "name": "Muhammad Lutfi Muzakii", "role": "Creator" },
            { "id": 2, "name": "Zenvu.js Core Developer", "role": "Maintainer" }
        ]
    }))
}

async fn create_user_handler(Json(payload): Json<Value>) -> Json<Value> {
    Json(json!({
        "success": true,
        "message": "User created successfully",
        "data": payload
    }))
}
