use std::sync::{Arc};
use dashmap::DashMap;
use warp::{Filter};
use memfast::{Core, ws_process};

#[tokio::main]
async fn main() {
    let core: Core = Arc::new(DashMap::new());
    let ws_core_clone = Arc::clone(&core);
    let _http_core_clone = Arc::clone(&core);

    // WebSocket Route
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let core = Arc::clone(&ws_core_clone);
            ws.on_upgrade(|websocket| ws_process(websocket, core))
        });

    // HTTP Route
    let http_route = warp::path("http")
        .and(warp::post())
        .map(|| "Hello!");

    let routes = ws_route.or(http_route);

    println!("Starting Memfast!");

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
