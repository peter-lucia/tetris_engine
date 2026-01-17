#[macro_use]
extern crate lazy_static;
use crate::well::{Tetris, Well};
use std::borrow::{Borrow, BorrowMut};

mod tetromino;
mod well;


fn main() {
    let mut t: Well = Tetris::new();
    t.setup();
    t.run_game();
}


