extern crate common;
mod well_command_line;

use std::io::{stdout, Write};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal,
    cursor,
    style::{self, Stylize},
    Result,
};
use crate::common::well::{Well};
use std::{io, thread, error::Error};
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
use crate::well_command_line::{CommandLine, WellCommandLine};

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
    let mut game: WellCommandLine = WellCommandLine::new();
    game.run();
    terminal::disable_raw_mode();
    Ok(())
}