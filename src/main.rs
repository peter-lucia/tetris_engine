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
use rocket::http::{Header, Status};
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::futures::stream;
use serde_json::{json};

#[get("/")]
fn default() -> &'static str {
    "You've reached the rust_tetris API homepage!"
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

fn remove_game(id: String) -> () {
    // the mutex is scoped so we don't actually have to manually remove it
    let mut map: MutexGuard<HashMap<String, Well>> = ACTIVE_GAMES.lock().unwrap();
    map.remove(&id.clone());
    log::info!("Stopped game with id {id}", id=id);
}

fn read_game(id: String) -> Well {
    let map: MutexGuard<HashMap<String, Well>> = ACTIVE_GAMES.lock().unwrap();
    let game: Well = map.get(&id.clone()).cloned().unwrap();
    std::mem::drop(map);
    return game;
}

#[get("/setup_game")]
fn new_game() -> String {
    log::info!("Starting setup of new game");
    let mut map: MutexGuard<HashMap<String, Well>> = ACTIVE_GAMES.lock().unwrap();
    let mut w: Well = Tetris::new();
    w.fall_delay_ms = 1000;
    let id: String = w.id.clone();
    map.insert(w.id.clone(),w.clone());
    std::mem::drop(map);
    run_with_mutex_mut(id.clone(), &Well::setup);
    log::info!("Starting setup of new game with id {id}", id=id);
    return serde_json::to_string(&w).unwrap();
}

/// Create a new game
#[get("/game_status")]
fn start_game() -> EventStream![] {
    let mut map: MutexGuard<HashMap<String, Well>> = ACTIVE_GAMES.lock().unwrap();
    let map_is_empty = map.is_empty();
    let mut id = String::from("");
    if !map_is_empty {
        id = map.keys().last().unwrap().to_string();
    }
    log::info!("Map is empty? {map_is_empty}", map_is_empty=map_is_empty);
    std::mem::drop(map);
    EventStream! {
        if id != "".to_string() {
            let mut running = read_game(id.clone()).running;
            let mut interval = time::interval(Duration::from_millis(read_game(id.clone()).fall_delay_ms));
            while running {
                running = run_with_mutex_mut(id.clone(), &Well::run_frame);
                let w: Well = read_game(id.clone());
                let game_state = serde_json::to_string(&w).unwrap();
                interval.tick().await;
                yield Event::data(game_state);
                let mut new_interval = time::interval(Duration::from_millis(read_game(id.clone()).fall_delay_ms));
                if interval.period() != new_interval.period() {
                    interval = new_interval;
                    interval.tick().await;
                }
            }
            run_with_mutex_mut(id.clone(), &Well::exit);
            let w: Well = read_game(id.clone());
            let game_state = serde_json::to_string(&w).unwrap();
            yield Event::data(game_state);
            remove_game(id.clone());
        } else {
            log::info!("Game completed.");
            yield Event::data(json!({
                "running": false,
                "data": {}
                }).to_string())
        }
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

/*
CORS Handling
 */
pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
        response.set_status(Status::Ok);
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![
            default,
            start_game,
            move_tetromino,
            rotate_tetromino,
            new_game]).attach(CORS)
}
