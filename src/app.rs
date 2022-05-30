use crate::tetromino::{Tetromino, TetrominoL};
use std::io;
use crossterm::{
    event::{self, Event, KeyCode},
};
use std::cmp::min;

/// App holds the state of the application
pub struct App {
    shape: Tetromino,
}

impl App {
    pub fn new() -> App {
        App {
            shape: Tetromino::make_l(),
        }
    }
}
