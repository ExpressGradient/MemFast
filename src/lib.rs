use dashmap::DashMap;
use reqwest;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::fs::{File, OpenOptions};
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;

pub type Core = Arc<DashMap<String, String>>;

// Core Process
pub async fn core_process(query: String, core: Core) -> String {
    let comps: Vec<&str> = query.splitn(10, " ").collect();

    match comps[0] {
        "GET" => get_key(core, comps[1].to_string()).await,
        "SET" => insert_key(core, comps[1].to_string(), comps[2].to_string()).await,
        "DEL" => delete_key(core, comps[1].to_string()).await,
        "EXISTS" => check_key(core, comps[1].to_string()).await,
        "LEN" => map_len(core).await,
        "ISEMPTY" => map_is_empty(core).await,
        "CLEAR" => clear_map(core).await,
        "DUMP" => dump_keys(core, comps[1]).await,
        "NET" => create_net(comps).await,
        "SETSYNC" => {
            insert_sync(
                comps[1].to_string(),
                comps[2].to_string(),
                comps[3].to_string(),
                core,
            )
            .await
        }
        _ => String::from("Command not implemented yet!"),
    }
}

// Get a Value using a Key
async fn get_key(core: Core, key: String) -> String {
    if let Some(value) = core.get(&*key) {
        value.value().clone()
    } else {
        String::from("Nil")
    }
}

// Insert or Update a Value
async fn insert_key(core: Core, key: String, value: String) -> String {
    if let Some(old_value) = core.insert(key, value) {
        old_value
    } else {
        String::from("Inserted!")
    }
}

// Delete a Key
async fn delete_key(core: Core, key: String) -> String {
    core.remove(&*key);
    String::from("Deleted!")
}

// Check existence of a Key
async fn check_key(core: Core, key: String) -> String {
    String::from(format!("{}", core.contains_key(&*key)))
}

// Get Length of the HashMap
async fn map_len(core: Core) -> String {
    String::from(format!("{}", core.len()))
}

// Check if the HashMap is Empty
async fn map_is_empty(core: Core) -> String {
    String::from(format!("{}", core.is_empty()))
}

// Clear the HashMap
async fn clear_map(core: Core) -> String {
    core.clear();
    String::from("Cleared!")
}

// Dump all Keys into a CSV file
async fn dump_keys(core: Core, path: &str) -> String {
    let mut file = File::create(path).await.unwrap();

    file.write_all("key, value\n".as_bytes()).await.unwrap();

    for ref_multi in core.iter() {
        let mut file = OpenOptions::new().append(true).open(path).await.unwrap();

        file.write_all(format!("{},{}\n", ref_multi.key(), ref_multi.value()).as_bytes())
            .await
            .unwrap();
    }

    String::from("Dumped!")
}

// Create a Network of Nodes
async fn create_net(comps: Vec<&str>) -> String {
    let name = comps[1].to_string();
    let num_nodes = comps[2].parse().unwrap();

    let mut ips = String::new();

    for i in 0..num_nodes {
        ips.push_str(comps[i + 3]);
        ips.push_str("_");
    }
    ips.pop();

    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(String::from("query"), format!("SET {} {}", name, ips));

    let client = reqwest::Client::new();

    for i in 0..num_nodes {
        client.post(comps[i + 3]).json(&map).send().await.unwrap();
    }

    sleep(Duration::from_secs(2)).await;

    String::from("Created!")
}

// Insert keys to be synced across nodes
async fn insert_sync(net: String, key: String, value: String, core: Core) -> String {
    if let Some(nodes) = core.get(&*net) {
        let ips: Vec<&str> = nodes.value().split("_").collect();

        for ip in ips {
            let mut map: HashMap<String, String> = HashMap::new();
            map.insert(String::from("query"), format!("SET {} {}", key, value));

            let client = reqwest::Client::new();
            client.post(ip).json(&map).send().await.unwrap();
        }
    }

    sleep(Duration::from_secs(2)).await;

    String::from("Synced!")
}
