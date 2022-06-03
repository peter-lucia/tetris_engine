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
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};

fn main() -> Result<()> {

    let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("log/output.log")?;

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
                   .appender("logfile")
                   .build(LevelFilter::Info)).unwrap();

    log4rs::init_config(config).unwrap();

    terminal::enable_raw_mode();
    let mut board: Well = BoardCommandLine::new();
    board.run();
    terminal::disable_raw_mode();
    Ok(())
}