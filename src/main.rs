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
use util::ACTIVE_GAMES;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;
mod tetromino;
mod well;
mod util;


#[get("/")]
fn default() -> &'static str {
    "You've reached the rust_tetris homepage!"
}
///
/// Display all active games on the page

#[post("/game")]
fn setup_game() -> String {
    let mut map: MutexGuard<HashMap<String, Well>> = ACTIVE_GAMES.lock().unwrap();
    let mut t: Well = Tetris::new();
    let serialized = serde_json::to_string(&t).unwrap();
    map.insert(t.id.clone(), t);
    return serialized;
}

/// Moves the current tetromino right
/// req: json data encoded as a str reference that contains the game id to modify
#[post("/move", data = "<req>")]
fn _move(req: &str) -> String {
    let id: Option<String> = util::extract_id(req);
    if id.is_none() {
        return util::get_response_missing_id_json(req);
    }
    let binding: serde_json::Value = serde_json::from_str(req).unwrap();
    let direction: String = binding.get("direction").unwrap().as_str().unwrap().to_string();
    let mut hashmap_guard: MutexGuard<HashMap<String, Well>> = ACTIVE_GAMES.lock().unwrap();
    // must clone the original reference
    let mut well: Well = hashmap_guard.get(&id.clone().unwrap()).cloned().unwrap();
    if direction == "left" {
        well.move_left();
    } else if direction == "right" {
        well.move_right();
    }
    // create the json response
    let result = serde_json::to_string(&well).unwrap();
    // update the active games
    hashmap_guard.insert(id.unwrap().clone().to_string(), well);
    return result;
}

/// Moves the current tetromino left
/// req: json data encoded as a str reference that contains the game id to modify
#[post("/rotate", data = "<req>")]
fn rotate(req: &str) -> String {
    let id: Option<String> = util::extract_id(req);
    if id.is_none() {
        return util::get_response_missing_id_json(req);
    }
    let binding: serde_json::Value = serde_json::from_str(req).unwrap();
    let reverse: bool = binding.get("reverse").unwrap().as_bool().unwrap();

    let mut hashmap_guard: MutexGuard<HashMap<String, Well>> = ACTIVE_GAMES.lock().unwrap();
    // must clone the original reference
    let mut well: Well = hashmap_guard.get(&id.clone().unwrap()).cloned().unwrap();
    if reverse {
        well.rotate_left();
    } else {
        well.rotate_right();
    }
    // create the json response
    let result = serde_json::to_string(&well).unwrap();
    // update the active games
    hashmap_guard.insert(id.unwrap().clone().to_string(), well);
    return result;
}

fn reset() -> () {
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![default, setup_game, _move])
}
