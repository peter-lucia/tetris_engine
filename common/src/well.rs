use crate::tetromino::{get_random_tetromino, Tetromino};
use rand::Rng;

pub const WELL_WIDTH: usize = 20;
pub const WELL_HEIGHT: usize = 20;


pub struct Well {
    pub grid: [[i32; WELL_WIDTH]; WELL_HEIGHT],
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

impl Default for Well {
    fn default() -> Well {
        Well {
            grid: [[0; WELL_WIDTH]; WELL_HEIGHT],
            current_tetromino: get_random_tetromino(),
            score: 0,
            running: true,
            fall_delay_ms: 1000,
            fall_delay_min_ms: 100,
            fall_delay_delta: 50,
        }
    }
}
