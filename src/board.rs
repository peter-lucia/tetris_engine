use crate::tetromino::Tetromino;
use rand::Rng;
use self::ndarray::Array2;
use std::thread::sleep;
use std::time::Duration;

extern crate ndarray;


pub struct Board {
    // https://docs.rs/ndarray/latest/ndarray/
    board: Array2<i32>,
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
    fn render(&self) -> ();
    fn run(&self) -> ();
    fn move_tetromino(&mut self, tetromino: Tetromino, direction: Direction) -> ();
    fn stick_tetromino(&mut self, tetromino: Tetromino) -> ();
}

impl BoardCommandLine for Board {

    fn new() -> Board {
        return Board {
            board: Array2::<i32>::zeros((60, 40)),
        }
    }
    /*
    Gradually increases the refresh rate, moving, the tetromino down a block faster with each
    finished epoch.
     */
    fn render(&self) -> () {
        println!("{:?}", self.board);
    }

    /*
    Render the tetris board
     */
    fn run(&self) -> () {
        let duration = Duration::new(1, 0);
        loop {
            self.render();
            sleep(duration);
        }
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

impl BoardBrowser for Board {
    fn render() -> () {

    }
}
