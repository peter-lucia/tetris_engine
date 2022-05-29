mod tetromino;
mod well;
mod app;

use std::io::{stdout, Write};
use crossterm::{
  ExecutableCommand, QueueableCommand,
  terminal, cursor, style::{self, Stylize}, Result
};
use well::{Well, BoardCommandLine};
use std::{io, thread, error::Error};
use app::{
    App,
};

fn run_tetris() {
}


fn main() -> Result<()> {
    let mut board: Well = BoardCommandLine::new();
    board.run();
    Ok(())
}