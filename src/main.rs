use std::sync::{Arc};
use dashmap::DashMap;
use warp::{Filter};
use memfast::{Core, http_process, ws_process};
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
struct Body {
    query: String,
}

#[derive(Serialize)]
struct Response {
    data: String,
    error: String,
}

#[tokio::main]
async fn main() {
    let core: Core = Arc::new(DashMap::new());
    let core_clone = Arc::clone(&core);

    // WebSocket Route
    let ws_route = warp::path("ws")
        .and(warp::ws())
        .map(move |ws: warp::ws::Ws| {
            let core = Arc::clone(&core);
            ws.on_upgrade(|websocket| ws_process(websocket, core))
        });

    // HTTP Route
    let http_route = warp::path("http")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 20))
        .and(warp::body::json())
        .map(move |body: Body| {
            let core = Arc::clone(&core_clone);
            let process_value = http_process(core, body.query);

            let mut response = Response { data: process_value.clone(), error: "".to_string() };

            if process_value == "Command not implemented!" {
                response.data = String::from("");
                response.error = process_value;
            }

            warp::reply::json(&response)
        });

    let routes = ws_route.or(http_route);

    println!("Starting Memfast!");
    println!("Serving WebSockets at ws://localhost:3030/ws");
    println!("Serving HTTP at http://localhost:3030/http");

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
