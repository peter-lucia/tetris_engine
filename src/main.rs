use std::borrow::{Borrow, BorrowMut};
use std::rc::Rc;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;
use crate::well::{Tetris, Well, WELL_HEIGHT, WELL_WIDTH};
use serde_json::{json, to_string};
use serde_json;
use uuid::{Uuid, uuid};
use std::collections::HashMap;
use rocket::http::RawStr;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;
mod tetromino;
mod well;

lazy_static! {
    static ref ACTIVE_GAMES: Mutex<HashMap<String, Well>> = {
        let mut m = HashMap::new();
        Mutex::new(m)
    };
}


#[get("/")]
fn default() -> &'static str {
    "You've reached the rust_tetris homepage!"
}

#[post("/game")]
fn setup_game() -> String {
    let mut map = ACTIVE_GAMES.lock().unwrap();
    let mut t: Well = Tetris::new();
    let id = Uuid::new_v4();
    let serialized = serde_json::to_string(&t).unwrap();
    map.insert(id.to_string(), t);
    return serialized;
}

#[post("/left")]
fn move_left() -> String {
    "OK".to_string()  // placeholder
}

#[post("/right", data = "<req>")]
fn move_right(req: &str) -> String {

    let binding: serde_json::Value = serde_json::from_str(req).unwrap();
    let id = binding.get("id").unwrap();
    let mut hashmap_guard: MutexGuard<HashMap<String, Well>> = ACTIVE_GAMES.lock().unwrap();
    if hashmap_guard.get(id.to_string().as_str()).is_none() {
        log::info!("Missing id {id}", id=id);
        return json!({
            "status": format!("Missing id: {id}", id = id),
            "data": {
                "id": id,
            }
        }).to_string();
    }
    // must clone the original reference
    let mut well: Well = hashmap_guard.get(&id.to_string()).cloned().unwrap();
    well.move_right();
    // create the json response
    let result = serde_json::to_string(&well).unwrap();
    // update the active games
    hashmap_guard.insert(id.to_string(), well);
    return result;
}

fn rotate_left() -> () {
}

fn rotate_right() -> () {
}

fn reset() -> () {
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![default, setup_game, move_right])
}
