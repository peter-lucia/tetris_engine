use std::borrow::{Borrow, BorrowMut};
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use crate::well::{Tetris, Well, WELL_HEIGHT, WELL_WIDTH};
#[macro_use] extern crate rocket;
use serde_json;

mod tetromino;
mod well;

struct Games {
    games: Vec<Well>
}

#[get("/")]
fn default() -> &'static str {
    "You've reached the rust_tetris homepage!"
}

#[post("/game")]
fn create_game() -> String {
    let mut t: Well = Tetris::new();
    let serialized = serde_json::to_string(&t).unwrap();
    return serialized;
}

// #[put("/game")]
// fn update_game(well_updated: String) -> String {
//     let deserialized: Well = serde_json::from_str(&well_updated).unwrap();
//     "OK".to_string()
// }


#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![default, create_game])
}
