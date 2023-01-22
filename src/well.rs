use crate::tetromino::{Tetromino, TetrominoL, TETROMINO_HEIGHT, TETROMINO_WIDTH, TetrominoStraight, get_random_tetromino};
use rand::Rng;
use std::time::Duration;
use std::io::Write;
use crate::tetromino;
use std::error::Error;
use std::cmp::{min, max};
use std::time::Instant;
use std::borrow::BorrowMut;
use std::{fs};
use std::any::Any;
use std::future::Future;
use std::ops::DerefMut;
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde_json::Result;
use uuid::{Uuid, uuid};


pub const WELL_WIDTH: usize = 14;
pub const WELL_HEIGHT: usize = 20;
const HIGH_SCORE_FILENAME: &str = "HIGH_SCORE";

fn get_x_offset() -> usize {
    return 0;
}

fn get_y_offset() -> usize {
    return 0;
}

#[derive(Serialize, Deserialize)]
pub struct Well {
    pub id: String,
    #[serde(skip, default="Instant::now")]
    current_instant: Instant,
    #[serde(skip, default="Instant::now")]
    last_instant: Instant,
    pub grid: [[i32; WELL_WIDTH]; WELL_HEIGHT],
    #[serde(skip)]
    pub current_tetromino: Tetromino,
    pub score: i32,
    pub running: bool,
    pub fall_delay_ms: u64,
    pub fall_delay_min_ms: u64,
    pub fall_delay_delta: u64,
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

impl Well {

    pub fn setup_game(&mut self) -> () {
        self.setup();
    }

    pub fn increment_frame(&mut self) -> () {
        self.run_frame();
    }

    pub fn move_down(&mut self) -> () {
        self.move_tetromino(Direction::Down);
    }

    pub fn move_left(&mut self) -> () {
        self.move_tetromino(Direction::Left);
    }

    pub fn move_right(&mut self) -> () {
        self.move_tetromino(Direction::Right);
    }

    fn rotate_left(&mut self) -> () {
        self.rotate_tetromino(true)
    }

    fn rotate_right(&mut self) -> () {
        self.rotate_tetromino(false)
    }

    fn exit(&mut self) -> () {
        self.quit();
    }
}


pub trait Tetris {
    /*
    pub is implied in traits
     */
    fn new() -> Self;
    fn render_edges_and_stuck_pieces(&mut self) -> ();
    fn render_score(&mut self, score: i32);
    fn render_game_status(&mut self, status: &str);
    fn record_high_score(&mut self) -> ();
    fn get_high_score(&self) -> i32;
    fn set_high_score(&self, high_score: i32) -> ();
    fn render_tetromino(&mut self, erase: bool) -> ();
    fn render_falling_blocks(&mut self) -> ();
    fn move_tetromino(&mut self, direction: Direction) -> ();
    fn log_grid(&self) -> ();
    fn quit(&mut self) -> ();
    fn setup(&mut self) -> ();
    fn run_frame(&mut self) -> bool;
    fn simulate_game(&mut self) -> ();
    fn rotate_tetromino(&mut self, reverse: bool) -> ();
}


impl Clone for Well {
    fn clone(&self) -> Self {
        Well {
            id: Uuid::new_v4().to_string(),
            current_instant: self.current_instant,
            last_instant: self.last_instant,
            grid: self.grid,
            current_tetromino: self.current_tetromino.clone(),
            score: self.score,
            running: false,
            fall_delay_ms: self.fall_delay_ms,
            fall_delay_min_ms: self.fall_delay_min_ms,
            fall_delay_delta: self.fall_delay_delta,
        }
    }
}
impl Tetris for Well {

    /// Creates a new well object
    fn new() -> Well {
        let mut result = Well {
            // |<---------- 12 --------->| plus 2 chars to display edge of wells = 14 x 20
            // where the well is of height 18 with two lines for the top (if needed) and bottom
            id: Uuid::new_v4().to_string(),
            grid: [[0; WELL_WIDTH]; WELL_HEIGHT],
            current_instant: Instant::now(),
            last_instant: Instant::now(),
            current_tetromino: get_random_tetromino(),
            score: 0,
            running: true,
            fall_delay_ms: 1000,
            fall_delay_min_ms: 100,
            fall_delay_delta: 50,
        };
        result.render_edges_and_stuck_pieces();
        result.render_score(result.score);

        return result;
    }

    fn setup(&mut self) -> () {
        println!("Game Starting...");
        self.render_edges_and_stuck_pieces();
        env_logger::init();
        println!("Initialized...")
    }

    fn run_frame(&mut self) -> bool {
        println!("Current position ({},{})", self.current_tetromino.x, self.current_tetromino.y);
        self.log_grid();
        if self.current_tetromino.is_stuck(self.grid) && self.current_tetromino.y != 0 {
            self.current_tetromino.stick_to_grid(&mut self.grid);
            log::info!("Current tetromino is stuck!");
            self.current_tetromino = get_random_tetromino();
        } else if self.current_tetromino.is_stuck(self.grid) && self.current_tetromino.y == 0 {
            return false;
        }
        else {
            self.move_tetromino(Direction::Down);
        }
        return true;
    }

    fn simulate_game(&mut self) -> () {
        self.setup();
        while self.running {
            self.running = self.run_frame();
        }
        self.quit();
    }


    fn render_edges_and_stuck_pieces(&mut self) -> () {
        // paint the outline of the board
        for x in 0..WELL_WIDTH {
            for y in 0..WELL_HEIGHT {
                if y == 0 || y == WELL_HEIGHT - 1 {
                    self.grid[y][x] = 1;
                }
                else if x == 0 || x == WELL_WIDTH - 1 {
                    self.grid[y][x] = 1;
                } else if self.grid[y][x] == 1 {
                }
                else {
                    self.grid[y][x] = 0;
                }
            }
        }

    }

    fn render_game_status(&mut self, status: &str) {
    }

    fn render_score(&mut self, score: i32) {
    }

    /// Render the tetromino 4x4 grid onto the tetris well
    /// Only the grid's walls and stuck tetrominos are marked as 1
    /// empty spaces, including the current tetromino, are left as 0 on the grid
    /// until they are stuck
    /// 2 on grid means tetrominomo is not stuck
    /// 1 on grid corresponds with a border or a stuck tetromino
    fn render_tetromino(&mut self, erase: bool) -> () {

        let x_min = self.current_tetromino.x;
        let x_max = self.current_tetromino.x + TETROMINO_WIDTH;
        let y_min = self.current_tetromino.y;
        let y_max = self.current_tetromino.y + TETROMINO_HEIGHT;
        for x in x_min..x_max {
            for y in y_min..y_max {
                let yy = max(0, y - self.current_tetromino.y);
                let xx = max(0, x - self.current_tetromino.x);
                if !erase && self.current_tetromino.area[yy][xx] == 1 {
                    self.grid[y][x] = 2;
                } else {
                    if y > 0 && y < WELL_HEIGHT - 1 && x > 0 && x < WELL_WIDTH - 1
                        && self.grid[y][x] == 2 {
                        self.grid[y][x] = 0;
                    }
                }
            }
        }
        self.log_grid();
    }

    /// Check if any row is full, if so, clear it and let blocks above fall down
    fn render_falling_blocks(&mut self) -> () {
        let mut blocks_falling: bool = false;
        for y in 1..self.grid.len()-1 {
            if self.grid[y] == [1; WELL_WIDTH] {
                blocks_falling = true;
                log::info!("Clearing row {}", y);
                self.score += 100;
                if self.fall_delay_ms > self.fall_delay_min_ms {
                    self.fall_delay_ms -= self.fall_delay_delta;
                }
                self.render_score(self.score);
                self.grid[y] = [0; WELL_WIDTH];
                self.grid[y][0] = 1;
                self.grid[y][WELL_WIDTH-1] = 1;
                for x in 1..self.grid[y].len()-1 {
                }
            }
        }
        while blocks_falling {
            // let blocks fall down
            blocks_falling = false;
            for y in 2..self.grid.len()-1 {
                for x in 1..self.grid[y].len()-1 {
                    if self.grid[y-1][x] == 1 && self.grid[y][x] == 0 {
                        self.grid[y-1][x] = 0;
                        self.grid[y][x] = 1;
                        blocks_falling = true;
                    }
                }
            }
        }
    }

    fn move_tetromino(&mut self, direction: Direction) -> () {
        println!("Prepare to move tetromino...");
        self.render_tetromino(true);
        match direction {
            Direction::Left => {
                if !self.current_tetromino.will_collide(self.grid, -1, 0) {
                    self.current_tetromino.x -= 1;
                }
            }
            Direction::Right => {
                if !self.current_tetromino.will_collide(self.grid, 1, 0) {
                    self.current_tetromino.x += 1;
                }
            }
            Direction::Down => {
                if !self.current_tetromino.will_collide(self.grid, 0, 1) {
                    self.current_tetromino.y += 1;
                } else if self.current_tetromino.y == 0 {
                    self.record_high_score();
                    self.render_game_status("Game Over!");
                    self.running = false;
                }
            }
            Direction::Up => {
                // if !self.current_tetromino.will_collide(self.grid, 0, -1) {
                //     self.current_tetromino.y -= 1;
                // }
            }
        }
        self.render_tetromino(false);
        self.render_falling_blocks();
        println!("Moved tetromino...");
    }

    /// Rotates as many times until no collision happens,
    /// which can be 360 degrees, where no rotation is possible.
    fn rotate_tetromino(&mut self, reverse: bool) -> () {
        self.render_tetromino(true);
        let mut i = 0;
        loop {
            self.current_tetromino.rotate(reverse);
            if (!self.current_tetromino
                .will_collide(self.grid, 0, 0)) || i == 4 {
                break;
            }
            i += 1;
        }
        self.render_tetromino(false);
    }

    fn record_high_score(&mut self) -> () {
        let mut high_score = self.get_high_score();
        if self.score > high_score {
            high_score = self.score
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
        for x in 0..self.grid.len() {
            log::info!("{:?}", self.grid[x]);
        }
    }

    fn quit(&mut self) -> () {

    }
}
