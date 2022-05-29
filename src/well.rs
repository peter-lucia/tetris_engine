use crate::tetromino::{
    Tetromino,
    TetrominoL,
};
use rand::Rng;
use std::thread::sleep;
use std::time::Duration;
use std::io::{Stdout, Write, stdout};
use crossterm::{
    QueueableCommand,
    cursor,
    style,
    terminal,
    ExecutableCommand,
    event::KeyCode
};
use crossterm::style::Stylize;
use crossterm::event::{poll, read, Event};
use crate::tetromino;

pub struct Well {
    grid: [[i32; 12+2]; 18+2],
    stdout: Stdout,
    current_tetromino: Tetromino,
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
    fn move_tetromino(&mut self, direction: Direction) -> ();
    fn stick_tetromino(&mut self) -> ();
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
            current_tetromino: Tetromino::make_l(),
        };
        return result;
    }
    /*
    Gradually increases the refresh rate, moving, the tetromino down a block faster with each
    finished epoch.
     */
    fn render(&mut self) -> () {

        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                let mut output = "█".black();

                if self.grid[i][j] == 1 ||
                    ((self.current_tetromino.x <= j && j <= self.current_tetromino.x) &&
                    (self.current_tetromino.y <= i && i <= self.current_tetromino.y) &&
                    self.current_tetromino.area[i - self.current_tetromino.y as usize][j - self.current_tetromino.x as usize] == 1)
                    {
                    output = "█".white();
                }
                self.stdout.queue(cursor::MoveTo(i as u16, j as u16));
                self.stdout.queue(style::PrintStyledContent(output));
                self.stdout.flush();
            }
        }
    }

    /*
    Render the tetris board
     */
    fn run(&mut self) -> () {
        loop {
            self.render();
            if poll(Duration::from_millis(500)).unwrap() {
                match read().unwrap() {
                    Event::Key(event) => {
                        if event.code == KeyCode::Char('q') {
                            // exit
                            return;
                        }
                        else if event.code == KeyCode::Left {
                        }
                        else if event.code == KeyCode::Right {
                        }
                        else if event.code == KeyCode::Down {
                        }
                        else if event.code == KeyCode::Up {
                        }
                    },
                    Event::Mouse(event) => {
                        println!("{:?}", event)
                    },
                    Event::Resize(width, height) => {
                        println!("New size {}x{}", width, height)
                    },
                }
            }
            let duration = Duration::from_millis(500);
            sleep(duration);
            self.move_tetromino(Direction::Down);
        }
    }

    fn move_tetromino(&mut self, direction: Direction) -> () {
        match direction {
            Direction::Left => {

            }
            Direction::Right => {

            }
            Direction::Down => {
                if self.current_tetromino.x < self.grid.len() {
                    self.current_tetromino.x += 1;
                }

            }
            Direction::Up => {

            }
        }


    }

    fn stick_tetromino(&mut self) -> () {
        todo!("Write the tetromino to the grid.");
        todo!("Select a new tetromino and put it at the top.");
        todo!("Increase the refresh rate slightly.");
    }

    fn quit(&mut self) -> () {

        self.stdout.flush();
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
