#[macro_use]
extern crate lazy_static;
use crate::well::{Tetris, Well};
use std::borrow::{Borrow, BorrowMut};

mod tetromino;
mod well;


fn main() {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();
    log::info!("Starting Tetris...");
    let mut t: Well = Tetris::new();
    t.setup();
    t.run_game();
}


