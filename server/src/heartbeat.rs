use once_cell::sync::Lazy;
use scorched::*;
use std::{collections::HashMap, sync::RwLock};

// Hashmap<id, last_heartbeat>
static HEARTBEATS: Lazy<RwLock<HashMap<u8, std::time::Instant>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

pub fn update_heartbeat(id: u8) {
    HEARTBEATS
        .write()
        .unwrap()
        .insert(id, std::time::Instant::now());
}

pub fn get_lowest_available_id() -> u8 {
    let mut id = 0;
    loop {
        if !HEARTBEATS.read().unwrap().contains_key(&id) {
            return id;
        }
        id += 1;
    }
}

pub fn get_clients() -> Vec<u8> {
    HEARTBEATS.read().unwrap().keys().cloned().collect()
}

pub fn heartbeat_thread() {
    loop {
        let heartbeats = HEARTBEATS.read().unwrap().clone();
        for heartbeat in heartbeats.iter() {
            if heartbeat.1.elapsed().as_secs() > 15 {
                log_this(LogData {
                    importance: LogImportance::Warning,
                    message: format!("Heartbeat for id {} has expired", heartbeat.0),
                });
                HEARTBEATS.write().unwrap().remove(heartbeat.0);
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
