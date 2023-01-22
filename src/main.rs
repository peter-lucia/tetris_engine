use std::borrow::{Borrow, BorrowMut};
use std::rc::Rc;
use std::sync::{Arc, Mutex, MutexGuard};
use std::thread;
use std::time::Duration;
use crate::well::{Tetris, Well, WELL_HEIGHT, WELL_WIDTH};
use serde_json::{json};
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
    static ref ACTIVE_GAMES: Mutex<HashMap<Uuid, Well>> = {
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
    map.insert(id, t);
    return serialized;
}

#[post("/left")]
fn move_left() -> String {
    "OK".to_string()  // placeholder
}

#[post("/right", data = "<id>")]
fn move_right(id: &RawStr) -> String {
    let unique_id: Uuid = Uuid::parse_str(id.as_str()).unwrap();
    let mut guard: MutexGuard<HashMap<Uuid, Well>> = ACTIVE_GAMES.lock().unwrap();
    // must clone the original reference
    let mut well: Well = guard.get(&unique_id).cloned().unwrap();
    well.move_right();
    // update the active games
    let result = serde_json::to_string(&well).unwrap();
    guard.insert(unique_id, well);
    result
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
        .mount("/", routes![default, setup_game])
}
