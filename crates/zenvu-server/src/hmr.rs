//! Hot Module Replacement (HMR) via WebSocket.

use axum::{extract::ws::{WebSocket, WebSocketUpgrade, Message}, response::Response};
use std::sync::Arc;
use tokio::sync::broadcast;

#[allow(dead_code)]
static mut HMR_TX: Option<Arc<broadcast::Sender<String>>> = None;

/// Initialize the HMR broadcast channel.
pub fn init_hmr() -> broadcast::Sender<String> {
    let (tx, _) = broadcast::channel(100);
    tx
}

/// Send an HMR update to all connected clients.
pub fn send_hmr_update(tx: &broadcast::Sender<String>, module: &str) {
    let msg = serde_json::json!({
        "type": "update",
        "module": module,
        "timestamp": std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap().as_millis()
    });
    let _ = tx.send(msg.to_string());
}

/// WebSocket handler for HMR connections.
pub async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    ws.on_upgrade(handle_ws)
}

async fn handle_ws(mut socket: WebSocket) {
    tracing::info!("HMR client connected");

    // Send initial connection confirmation
    let welcome = serde_json::json!({
        "type": "connected",
        "version": env!("CARGO_PKG_VERSION"),
    });
    let _ = socket.send(Message::Text(welcome.to_string().into())).await;

    // Keep connection alive
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Ping(data) => {
                let _ = socket.send(Message::Pong(data)).await;
            }
            Message::Close(_) => break,
            _ => {}
        }
    }

    tracing::info!("HMR client disconnected");
}
