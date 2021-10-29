use dashmap::DashMap;
use std::sync::{Arc};
use futures_util::{SinkExt, StreamExt};
use warp::ws::{Message, WebSocket};

pub type Core = Arc<DashMap<String, String>>;

pub async fn ws_process(websocket: WebSocket, core: Core) {
    // Split the WebSocket into tx and rx.
    let (mut tx, mut rx) = websocket.split();

    // Continuously loop for Items by calling next() on the rx Stream.
    while let Some(item) = rx.next().await {
        match item {
            Ok(message) => {
                // Collect the Query Args.
                let query: Vec<&str> = message.to_str().unwrap().splitn(5, " ").collect();

                // Process the Query.
                let process_value = core_process(core.clone(), query).await;

                // Send the Value back to the Client.
                tx.send(Message::text(process_value)).await.unwrap();
            }
            Err(error) => {
                // If any error, print back to the console.
                eprintln!("{}", error);
                break;
            }
        };
    }
}

async fn core_process(core: Core, query: Vec<&str>) -> String {
    match query[0] {
        "GET" => {
            if let Some(dash_value) = core.get(query[1]) {
                dash_value.value().clone()
            } else {
                String::from("Nil")
            }
        }
        "SET" => {
            core.insert(String::from(query[1]), String::from(query[2]));
            String::from("Ok")
        }
        &_ => {
            String::from("Command not implemented!")
        }
    }
}
