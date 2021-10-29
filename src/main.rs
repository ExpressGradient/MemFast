use futures_util::{FutureExt, SinkExt, StreamExt};
use warp::{Error, Filter};
use warp::ws::{Message, WebSocket};

#[tokio::main]
async fn main() {
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(|ws: warp::ws::Ws| {
            ws.on_upgrade(|websocket| ws_process(websocket))
        });

    let http_route = warp::path("http")
        .and(warp::get())
        .map(|| "Hello!");

    let routes = ws_route.or(http_route);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

async fn ws_process(websocket: WebSocket) {
    let (mut tx, mut rx) = websocket.split();

    while let Some(item) = rx.next().await {
        match item {
            Ok(message) => {
                tx.send(Message::text("Message received")).await.unwrap();
            },
            Err(error) => {
                println!("{}", error);
                break;
            }
        };
    }
}
