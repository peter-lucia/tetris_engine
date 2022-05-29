mod tetromino;
mod well;
mod app;

use std::io::{stdout, Write};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal,
    cursor,
    style::{self, Stylize},
    Result,
};
use well::{Well, BoardCommandLine};
use std::{io, thread, error::Error};
use app::{
    App,
};


fn main() -> Result<()> {
    terminal::enable_raw_mode();
    let mut board: Well = BoardCommandLine::new();
    board.run();
    terminal::disable_raw_mode();
    Ok(())
}