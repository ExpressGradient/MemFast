use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Extension,
    },
    response::IntoResponse,
    routing::get,
    AddExtensionLayer, Json, Router,
};
use dashmap::DashMap;
use futures::{sink::SinkExt, stream::StreamExt};
use memfast::{core_process, Core};
use serde::Deserialize;
use std::env;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Core App State
    let dashmap: DashMap<String, String> = DashMap::new();
    let core = Arc::new(dashmap);

    // Core App
    let app = Router::new()
        .route("/", get(ws_handler).post(http_handler))
        .layer(AddExtensionLayer::new(core));

    let port: Vec<String> = env::args().collect();

    println!("Starting MemFast!!!");
    println!("WebSockets at ws://localhost:{}/", port[1]);
    println!("Serverless at http://localhost:{}/", port[1]);

    // Start App Server
    axum::Server::bind(&format!("0.0.0.0:{}", port[1]).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade, Extension(state): Extension<Core>) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket_handle(socket, state))
}

async fn websocket_handle(socket: WebSocket, state: Core) {
    let (mut sender, mut receiver) = socket.split();
    let state_clone = Arc::clone(&state);

    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(query) = message {
            let response = core_process(query, state_clone.clone()).await;
            sender
                .send(Message::Text(String::from(response)))
                .await
                .unwrap();
        }
    }
}

#[derive(Deserialize)]
struct JSONPayload {
    query: String,
}

async fn http_handler(
    Json(payload): Json<JSONPayload>,
    Extension(state): Extension<Core>,
) -> impl IntoResponse {
    let response = core_process(payload.query, state).await;
    response.into_response()
}
