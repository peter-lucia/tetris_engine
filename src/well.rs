use crate::tetromino::Tetromino;
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;
use std::io::{Stdout, Write, stdout};
use crossterm::{QueueableCommand, cursor, style, ExecutableCommand, terminal};
use crossterm::style::Stylize;

pub struct Well {
    // https://docs.rs/ndarray/latest/ndarray/
    grid: [[i32; 12+2]; 18+2],
    stdout: Stdout,
}

pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

pub fn random_direction() -> Direction {
    let mut rng = rand::thread_rng();
    match rng.gen_range(0..4) {
        0 => Direction::Up,
        1 => Direction::Down,
        2 => Direction::Left,
        _ => Direction::Right,
    }
}

pub trait BoardCommandLine {
    /*
    pub is implied in traits
     */
    fn new() -> Self;
    fn render(&mut self) -> ();
    fn run(&mut self) -> ();
    fn move_tetromino(&mut self, tetromino: Tetromino, direction: Direction) -> ();
    fn stick_tetromino(&mut self, tetromino: Tetromino) -> ();
    fn quit(&mut self) -> ();
}

impl BoardCommandLine for Well {

    fn new() -> Well {
        let mut stdout = stdout();
        stdout.execute(terminal::Clear(terminal::ClearType::All));
        let result = Well {
            // |<---------- 12 --------->| plus 2 chars to display edge of wells = 14 x 20
            // where the well is of height 18 with two lines for the top (if needed) and bottom
            grid: [[0; 14] ; 20],
            stdout: stdout,
        };
        return result;
    }
    /*
    Gradually increases the refresh rate, moving, the tetromino down a block faster with each
    finished epoch.
     */
    fn render(&mut self) -> () {
        // println!("{:?}", self.grid);
        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                let mut output = "█".black();

                if self.grid[i][j] == 1 {
                    output = "█".magenta();
                }
                self.stdout.queue(cursor::MoveTo(i as u16, j as u16));
                self.stdout.queue(style::PrintStyledContent(output));
            }
        }
    }

    /*
    Render the tetris board
     */
    fn run(&mut self) -> () {
        let duration = Duration::new(1, 0);
        loop {
            self.render();
            sleep(duration);
        }
    }

    fn quit(&mut self) -> () {

        self.stdout.flush();
    }

    fn move_tetromino(&mut self, tetromino: Tetromino, direction: Direction) -> () {

    }

    fn stick_tetromino(&mut self, tetromino: Tetromino) -> () {
        todo!()
    }
}

pub trait BoardBrowser {
    /*
    pub is implied in traits
     */
    fn render() -> ();
}

impl BoardBrowser for Well {
    fn render() -> () {

    }
}
