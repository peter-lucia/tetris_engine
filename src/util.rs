use serde_json::{json, to_string};
use serde_json;
use std::sync::{Arc, Mutex, MutexGuard};
use std::collections::HashMap;
use crate::well::{Tetris, Well, WELL_HEIGHT, WELL_WIDTH};

lazy_static! {
    pub static ref ACTIVE_GAMES: Mutex<HashMap<String, Well>> = {
        let mut m = HashMap::new();
        Mutex::new(m)
    };
}


pub fn extract_id(req: &str) -> Option<String> {
    let mut hashmap_guard: MutexGuard<HashMap<String, Well>> = ACTIVE_GAMES.lock().unwrap();
    let binding: serde_json::Value = serde_json::from_str(req).unwrap();
    let id: String = binding.get("id").unwrap().as_str().unwrap().to_string();
    if !hashmap_guard.contains_key(&id) {
        return None;
    }
    return Option::from(id);
}

pub fn get_response_missing_id_json(req: &str) -> String {
    let binding: serde_json::Value = serde_json::from_str(req).unwrap();
    let id: String = binding.get("id").unwrap().as_str().unwrap().to_string();
    log::info!("Missing id {id}", id=id);
    return json!({
            "status": format!("Missing id: {id}", id = id),
            "data": {}
        }).to_string();
}
