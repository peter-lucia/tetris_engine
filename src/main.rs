use crate::board::{Board, BoardCommandLine};

mod board;
mod tetromino;

fn main() {
    println!("Hello, world!");
    let board: Board = BoardCommandLine::new();
    board.run()
}
