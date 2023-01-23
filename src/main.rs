#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rocket;

use std::collections::HashMap;
use std::sync::MutexGuard;

use serde_json;
use util::ACTIVE_GAMES;

use crate::well::{Tetris, Well};

mod tetromino;
mod well;
mod util;

use rocket::response::stream::{Event, EventStream};
use rocket::tokio::time::{self, Duration};

#[get("/")]
fn default() -> &'static str {
    "You've reached the rust_tetris homepage!"
}

fn run_with_mutex_mut<T>(id: String, func: &dyn Fn(&mut Well) -> T) -> T {
    let mut map: MutexGuard<HashMap<String, Well>> = ACTIVE_GAMES.lock().unwrap();
    let mut game: Well = map.get(&id.clone()).cloned().unwrap();
    let res = func(&mut game);
    game.log_grid();
    map.insert(game.id.clone(), game.clone());
    std::mem::drop(map);
    return res;
}

fn read_game(id: String) -> Well {
    let map: MutexGuard<HashMap<String, Well>> = ACTIVE_GAMES.lock().unwrap();
    let game: Well = map.get(&id.clone()).cloned().unwrap();
    std::mem::drop(map);
    return game;
}

/// Create a new game
#[post("/game")]
fn setup_game() -> EventStream![] {
    let mut map: MutexGuard<HashMap<String, Well>> = ACTIVE_GAMES.lock().unwrap();
    let mut t: Well = Tetris::new();
    let id: String = t.id.clone();
    map.insert(t.id.clone(),t.clone());
    let mut game: Well = map.get(&t.id.clone()).cloned().unwrap();
    std::mem::drop(map);
    EventStream! {
        run_with_mutex_mut(id.clone(), &Well::setup);
        let mut interval = time::interval(Duration::from_millis(game.fall_delay_ms));
        let mut running = true;
        while running {
            run_with_mutex_mut(id.clone(), &Well::move_down);
            running = run_with_mutex_mut(id.clone(), &Well::run_frame);
            let t: Well = read_game(id.clone());
            // t.log_grid();
            let game_state = serde_json::to_string(&t).unwrap();
            yield Event::data(game_state);
            interval.tick().await;
        }
        run_with_mutex_mut(id.clone(), &Well::quit);
    }
}

/// Move the tetromino
/// Body Params:
/// {
///     "id": "1c582c72-e3dc-4999-a9f4-b5bc1fdfb394",
///     "direction": "left"
/// }
#[put("/move_tetromino", data = "<req>")]
fn move_tetromino(req: &str) -> String {
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

/// Rotate the tetromino
/// Body Params:
/// {
///     "id": "1c582c72-e3dc-4999-a9f4-b5bc1fdfb394",
///     "reverse": true
/// }
#[put("/rotate_tetromino", data = "<req>")]
fn rotate_tetromino(req: &str) -> String {
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
        .mount("/", routes![default, setup_game, move_tetromino, rotate_tetromino])
}
