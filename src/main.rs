use crate::board::{Board, BoardCommandLine};
mod board;
mod tetromino;

fn main() {
    let board: Board = BoardCommandLine::new();
    board.run()
}
