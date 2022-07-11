use crate::common::tetromino::{get_random_tetromino, Tetromino, TETROMINO_HEIGHT, TETROMINO_WIDTH, TetrominoL, TetrominoStraight};
use std::thread::sleep;
use std::time::Duration;
use std::io::{Stdout, stdout, Write};
use crossterm::{
    cursor,
    event::KeyCode,
    ExecutableCommand,
    QueueableCommand,
    style,
    terminal
};
use crossterm::style::{StyledContent, Stylize};
use crossterm::event::{Event, poll, read};
use crate::common::tetromino;
use crate::common::well;
use crate::common::well::{Direction, Well, WELL_HEIGHT, WELL_WIDTH};
use std::error::Error;
use std::cmp::{max, min};
use std::time::Instant;
use std::borrow::BorrowMut;
use std::fs;
use std::path::Path;

const HIGH_SCORE_FILENAME: &str = "HIGH_SCORE";

macro_rules! cmdline_color_white {
    () => {
        "█".white();
    }
}
macro_rules! cmdline_color_black {
    () => {
        " ".black();
    }
}

fn get_x_offset() -> usize {
    return ((terminal::size().unwrap().0 / 2) as usize - (WELL_WIDTH / 2)) as usize;
}

fn get_y_offset() -> usize {
    return (terminal::size().unwrap().1 / 4) as usize;
}

pub struct WellCommandLine {
    well: Well,
    stdout: Stdout,
}

pub trait CommandLine {
    /*
    pub is implied in traits
     */
    fn new() -> WellCommandLine;

    fn run(&mut self) -> crossterm::Result<()>;
    fn move_tetromino(&mut self, direction: Direction) -> ();
    fn render_tetromino(&mut self, erase: bool) -> ();
    fn render_falling_blocks(&mut self) -> ();
    fn render_edges_and_stuck_pieces(&mut self) -> ();
    fn render_score(&mut self, score: i32);
    fn render_game_status(&mut self, status: &str);
    fn write_to_stdout(&mut self, x: usize, y: usize, style: StyledContent<&str>);

    fn record_high_score(&mut self) -> ();
    fn get_high_score(&self) -> i32;
    fn set_high_score(&self, high_score: i32) -> ();

    fn log_grid(&self) -> ();
    fn quit(&mut self) -> ();
}


impl CommandLine for WellCommandLine {

    /// Creates a new command line well
    fn new() -> WellCommandLine {
        let mut stdout = stdout();
        stdout.queue(cursor::Hide);
        stdout.execute(terminal::Clear(terminal::ClearType::All));
        let mut result = WellCommandLine {
            stdout: stdout,
            well: Well::default()
        };
        result.render_edges_and_stuck_pieces();
        result.render_score(result.well.score);

        return result;
    }

    /// Gradually increases the refresh rate, moving the tetromino down a block faster with each
    /// finished epoch.
    fn run(&mut self) -> crossterm::Result<()> {
        let mut last_instant = Instant::now();
        while self.well.running {
            let current_instant= Instant::now();
            if current_instant.duration_since(last_instant) > Duration::from_millis(self.well.fall_delay_ms) {
                last_instant = current_instant;
                log::info!("Current position ({},{})", self.well.current_tetromino.x, self.well.current_tetromino.y);
                if self.well.current_tetromino.is_stuck(self.well.grid) && self.well.current_tetromino.y != 0 {
                    self.well.current_tetromino.stick_to_grid(&mut self.well.grid);
                    log::info!("Current tetromino is stuck!");
                    self.well.current_tetromino = get_random_tetromino();
                } else {
                    self.move_tetromino(Direction::Down);
                }
            }
            if poll(Duration::from_millis(1))? {
                match read().unwrap() {
                    Event::Key(event) => {
                        if event.code == KeyCode::Char('q') {
                            self.record_high_score();
                            self.render_game_status("Game Over!");
                            self.stdout.execute(terminal::Clear(terminal::ClearType::All));
                            // exit
                            return Ok(());
                        }
                        else if event.code == KeyCode::Left
                            || event.code == KeyCode::Char('h')
                            || event.code == KeyCode::Char('a') {
                            self.move_tetromino(Direction::Left);
                        }
                        else if event.code == KeyCode::Right
                            || event.code == KeyCode::Char('l')
                            || event.code == KeyCode::Char('d') {
                            self.move_tetromino(Direction::Right);
                        }
                        else if event.code == KeyCode::Down
                            || event.code == KeyCode::Char('j')
                            || event.code == KeyCode::Char('s') {
                            self.move_tetromino(Direction::Down);
                        }
                        else if event.code == KeyCode::Up
                            || event.code == KeyCode::Char('k')
                            || event.code == KeyCode::Char('w') {
                            self.move_tetromino(Direction::Up);
                        }
                        else if event.code == KeyCode::Char('r')
                            || event.code == KeyCode::Char(' ') {
                            self.render_tetromino(true);
                            let mut i = 0;
                            loop {
                                self.well.current_tetromino.rotate(false);
                                if (!self.well.current_tetromino
                                    .will_collide(self.well.grid, 0, 0)) || i == 4 {
                                    break;
                                }
                                i += 1;
                            }
                            self.render_tetromino(false);
                        }
                    },
                    Event::Mouse(event) => {
                        log::info!("{:?}", event);
                    },
                    Event::Resize(width, height) => {
                        log::info!("New size {}x{}", width, height);
                        let result = self.stdout.execute(
                            terminal::Clear(terminal::ClearType::All)
                        );
                        self.render_edges_and_stuck_pieces();
                        self.render_score(self.well.score);
                    },
                }
            }
        }
        return crossterm::Result::Ok(());
    }

    fn move_tetromino(&mut self, direction: Direction) -> () {
        self.render_tetromino(true);
        match direction {
            Direction::Left => {
                if !self.well.current_tetromino.will_collide(self.well.grid, -1, 0) {
                    self.well.current_tetromino.x -= 1;
                }
            }
            Direction::Right => {
                if !self.well.current_tetromino.will_collide(self.well.grid, 1, 0) {
                    self.well.current_tetromino.x += 1;
                }
            }
            Direction::Down => {
                if !self.well.current_tetromino.will_collide(self.well.grid, 0, 1) {
                    self.well.current_tetromino.y += 1;
                } else if self.well.current_tetromino.y == 0 {
                    self.record_high_score();
                    self.render_game_status("Game Over!");
                    self.well.running = false;
                }
            }
            Direction::Up => {
                // if !self.well.current_tetromino.will_collide(self.well.grid, 0, -1) {
                //     self.well.current_tetromino.y -= 1;
                // }
            }
        }
        self.render_tetromino(false);
        self.render_falling_blocks();
    }

    /// Render the tetromino 4x4 grid onto the tetris well
    /// Only the grid's walls and stuck tetrominos are marked as 1
    /// empty spaces, including the current tetromino, are left as 0 on the grid
    /// until they are stuck
    /// 2 on grid means tetrominomo is not stuck
    /// 1 on grid corresponds with a border or a stuck tetromino
    fn render_tetromino(&mut self, erase: bool) -> () {

        let x_min = self.well.current_tetromino.x;
        let x_max = self.well.current_tetromino.x + TETROMINO_WIDTH;
        let y_min = self.well.current_tetromino.y;
        let y_max = self.well.current_tetromino.y + TETROMINO_HEIGHT;
        for x in x_min..x_max {
            for y in y_min..y_max {
                let yy = max(0, y - self.well.current_tetromino.y);
                let xx = max(0, x - self.well.current_tetromino.x);
                if !erase && self.well.current_tetromino.area[yy][xx] == 1 {
                    self.well.grid[y][x] = 2;
                    self.write_to_stdout(x, y, cmdline_color_white!());
                } else {
                    if y > 0 && y < WELL_HEIGHT - 1 && x > 0 && x < WELL_WIDTH - 1
                        && self.well.grid[y][x] == 2 {
                        self.well.grid[y][x] = 0;
                        self.write_to_stdout(x, y, cmdline_color_black!());
                    }
                }
                self.stdout.flush();
            }
        }
        self.log_grid();
    }

    /// Check if any row is full, if so, clear it and let blocks above fall down
    fn render_falling_blocks(&mut self) -> () {
        let mut blocks_falling: bool = false;
        for y in 1..self.well.grid.len()-1 {
            if self.well.grid[y] == [1; WELL_WIDTH] {
                blocks_falling = true;
                log::info!("Clearing row {}", y);
                self.well.score += 100;
                if self.well.fall_delay_ms > self.well.fall_delay_min_ms {
                    self.well.fall_delay_ms -= self.well.fall_delay_delta;
                }
                self.render_score(self.well.score);
                self.well.grid[y] = [0; WELL_WIDTH];
                self.well.grid[y][0] = 1;
                self.well.grid[y][WELL_WIDTH-1] = 1;
                for x in 1..self.well.grid[y].len()-1 {
                    self.write_to_stdout(x, y, cmdline_color_black!());
                }
                self.stdout.flush();
            }
        }
        while blocks_falling {
            // let blocks fall down
            blocks_falling = false;
            for y in 2..self.well.grid.len()-1 {
                for x in 1..self.well.grid[y].len()-1 {
                    if self.well.grid[y-1][x] == 1 && self.well.grid[y][x] == 0 {
                        self.well.grid[y-1][x] = 0;
                        self.well.grid[y][x] = 1;
                        self.write_to_stdout(x, y - 1, cmdline_color_black!());
                        self.write_to_stdout(x, y, cmdline_color_white!());
                        self.stdout.flush();
                        blocks_falling = true;
                    }
                }
            }
        }
    }

    fn render_edges_and_stuck_pieces(&mut self) -> () {
        // paint the outline of the board
        let mut output = cmdline_color_black!();
        for x in 0..WELL_WIDTH {
            for y in 0..WELL_HEIGHT {
                if y == 0 || y == WELL_HEIGHT - 1 {
                    output = cmdline_color_white!();
                    self.well.grid[y][x] = 1;
                }
                else if x == 0 || x == WELL_WIDTH - 1 {
                    output = cmdline_color_white!();
                    self.well.grid[y][x] = 1;
                } else if self.well.grid[y][x] == 1 {
                    output = cmdline_color_white!()
                }
                else {
                    output = cmdline_color_black!();
                    self.well.grid[y][x] = 0;
                }
                self.write_to_stdout(x, y, output);
                self.stdout.flush();
            }
        }

    }

    fn render_score(&mut self, score: i32) {
        let x = get_x_offset() - 6;
        let y = get_y_offset() - 2;
        self.stdout.queue(cursor::MoveTo((x) as u16, (y) as u16)); // must be reversed
        let current_score = format!("Current Score: {} High Score: {}",
                                    score,
                                    self.get_high_score());
        self.stdout.queue(style::Print(current_score));
    }

    fn render_game_status(&mut self, status: &str) {
        let x = get_x_offset() - 6;
        let y = get_y_offset() - 4;
        self.stdout.queue(cursor::MoveTo((x) as u16, (y) as u16)); // must be reversed
        self.stdout.queue(style::Print(status.to_string()));
    }

    fn write_to_stdout(&mut self, x: usize, y: usize, style: StyledContent<&str>) {
        self.stdout.queue(cursor::MoveTo((x+get_x_offset()) as u16, (y+get_y_offset()) as u16)); // must be reversed
        self.stdout.queue(style::PrintStyledContent(style));
    }

    fn record_high_score(&mut self) -> () {
        let mut high_score = self.get_high_score();
        if self.well.score > high_score {
            high_score = self.well.score
        }
        self.set_high_score(high_score);
    }


    fn get_high_score(&self) -> i32 {
        let mut high_score = 0;
        if Path::new(HIGH_SCORE_FILENAME).exists() {
            high_score = fs::read_to_string(HIGH_SCORE_FILENAME)
                .unwrap().parse().unwrap();
        }
        return high_score;
    }

    fn set_high_score(&self, high_score: i32) -> () {
        fs::write(HIGH_SCORE_FILENAME, high_score.to_string());
    }

    fn log_grid(&self) -> () {
        log::info!("Grid: ");
        for x in 0..self.well.grid.len() {
            log::info!("{:?}", self.well.grid[x]);
        }
    }

    fn quit(&mut self) -> () {

        self.stdout.flush();
    }
}
