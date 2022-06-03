use crate::tetromino::{
    Tetromino,
    TetrominoL,
    TETROMINO_HEIGHT,
    TETROMINO_WIDTH,
    TetrominoStraight,
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
use crossterm::style::{Stylize, StyledContent};
use crossterm::event::{poll, read, Event};
use crate::tetromino;
use std::error::Error;
use std::cmp::{min, max};

pub const WELL_WIDTH: usize = 14;
pub const WELL_HEIGHT: usize = 20;

pub struct Well {
    grid: [[i32; WELL_WIDTH]; WELL_HEIGHT],
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
    fn render(&mut self, erase: bool) -> ();
    fn run(&mut self) -> crossterm::Result<()>;
    fn move_tetromino(&mut self, direction: Direction) -> ();
    fn stick_tetromino(&mut self) -> ();
    fn quit(&mut self) -> ();
}

impl BoardCommandLine for Well {

    /// Creates a new well for command line
    fn new() -> Well {
        let mut stdout = stdout();
        stdout.queue(cursor::Hide);
        stdout.execute(terminal::Clear(terminal::ClearType::All));
        let mut result = Well {
            // |<---------- 12 --------->| plus 2 chars to display edge of wells = 14 x 20
            // where the well is of height 18 with two lines for the top (if needed) and bottom
            grid: [[0; WELL_WIDTH] ; WELL_HEIGHT],
            stdout: stdout,
            current_tetromino: Tetromino::make_l(),
        };
        // paint the outline of the board
        let mut output = " ".black();
        for i in 0..WELL_HEIGHT{
            for j in 0..WELL_WIDTH {
                if i == 0 || i == WELL_HEIGHT - 1 {
                    output = "█".white();
                    result.grid[i][j] = 1;
                }
                else if j == 0 || j == WELL_WIDTH - 1 {
                    output = "█".white();
                    result.grid[i][j] = 1;
                } else {
                    output = " ".black();
                    result.grid[i][j] = 0;
                }
                result.stdout.queue(cursor::MoveTo(i as u16, j as u16));
                result.stdout.queue(style::PrintStyledContent(output));
                result.stdout.flush();
            }
        }
        return result;
    }

    /// Render the tetromino 4x4 grid onto the tetris well
    /// Only the grid's walls and stuck tetrominos are marked as 1
    /// empty spaces, including the current tetromino, are left as 0 on the grid
    /// until they are stuck
    fn render(&mut self, erase: bool) -> () {

        let x_min = self.current_tetromino.y;
        let x_max = self.current_tetromino.y + TETROMINO_HEIGHT;
        let y_min = self.current_tetromino.x;
        let y_max = self.current_tetromino.x + TETROMINO_WIDTH;
        for i in x_min..x_max {
            for j in y_min..y_max {
                let ii = max(0, i - self.current_tetromino.y);
                let jj = max(0, j - self.current_tetromino.x);
                if !erase && self.current_tetromino.area[ii][jj] == 1 {
                    // self.grid[i][j] = 1;
                    self.stdout.queue(cursor::MoveTo(i as u16, j as u16));
                    self.stdout.queue(style::PrintStyledContent("█".white()));
                } else {
                    // self.grid[i][j] = 0;
                    if i > 0 && i < WELL_HEIGHT - 1 && j > 0 && j < WELL_WIDTH - 1 {
                        self.stdout.queue(cursor::MoveTo(i as u16, j as u16));
                        self.stdout.queue(style::PrintStyledContent(" ".white()));
                    }
                }
                self.stdout.flush();
            }
        }
    }

    /// Gradually increases the refresh rate, moving, the tetromino down a block faster with each
    /// finished epoch.
    fn run(&mut self) -> crossterm::Result<()> {
        loop {
            if poll(Duration::from_millis(5))? {
                match read().unwrap() {
                    Event::Key(event) => {
                        if event.code == KeyCode::Char('q') {
                            self.stdout.execute(terminal::Clear(terminal::ClearType::FromCursorUp));
                            // exit
                            return Ok(());
                        }
                        else if event.code == KeyCode::Left || event.code == KeyCode::Char('h') {
                            self.move_tetromino(Direction::Left);
                        }
                        else if event.code == KeyCode::Right || event.code == KeyCode::Char('l') {
                            self.move_tetromino(Direction::Right);
                        }
                        else if event.code == KeyCode::Down || event.code == KeyCode::Char('j') {
                            self.move_tetromino(Direction::Down);
                        }
                        else if event.code == KeyCode::Up || event.code == KeyCode::Char('k') {
                            self.move_tetromino(Direction::Up);
                        }
                        else if event.code == KeyCode::Char('r') {
                            self.render(true);
                            self.current_tetromino.rotate(false);
                            self.render(false);
                            // if there's already a collision, undo the rotation
                            if self.current_tetromino.will_collide(self.grid, 0, 0) {
                                self.render(true);
                                self.current_tetromino.rotate(true);
                                self.render(false);
                            }
                        }
                    },
                    Event::Mouse(event) => {
                        log::info!("{:?}", event);
                    },
                    Event::Resize(width, height) => {
                        log::info!("New size {}x{}", width, height);
                    },
                }
            }
        }
    }

    fn move_tetromino(&mut self, direction: Direction) -> () {
        self.render(true);
        match direction {
            Direction::Left => {
                if !self.current_tetromino.will_collide(self.grid, 0, -1) {
                    self.current_tetromino.y -= 1;
                }
            }
            Direction::Right => {
                if !self.current_tetromino.will_collide(self.grid, 0, 1) {
                    self.current_tetromino.y += 1;
                }
            }
            Direction::Down => {
                if !self.current_tetromino.will_collide(self.grid, 1, 0) {
                    self.current_tetromino.x += 1;
                }
            }
            Direction::Up => {
                if !self.current_tetromino.will_collide(self.grid, -1, 0) {
                    self.current_tetromino.x -= 1;
                }
            }
        }
        self.render(false);
        log::info!("Current position ({},{})", self.current_tetromino.x, self.current_tetromino.y);
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
