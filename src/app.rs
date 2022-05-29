use crate::tetromino::{Tetromino, TetrominoL};
use std::io;
use crossterm::{
    event::{self, Event, KeyCode},
};
use std::cmp::min;

// https://github.com/fdehau/tui-rs/blob/v0.18.0/examples/canvas.rs
/// App holds the state of the application
pub struct App {
    x: i32,
    y: i32,
    shape: Tetromino,
}

impl App {
    pub fn new() -> App {
        App {
            x: 0,
            y: 0,
            shape: Tetromino::make_l(),
        }
    }
}
