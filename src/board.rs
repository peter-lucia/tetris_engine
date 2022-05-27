use crate::tetromino::Tetromino;
use rand::Rng;

pub struct Board {
    // https://docs.rs/ndarray/latest/ndarray/
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
    fn render() -> ();
    fn move_tetromino(&mut self, tetromino: Tetromino, direction: Direction) -> ();
    fn stick_tetromino(&mut self, tetromino: Tetromino) -> ();
}

impl BoardCommandLine for Board {
    /*
    Gradually increases the refresh rate, moving, the tetromino down a block faster with each
    finished epoch.
     */
    fn render() -> () {
        todo!()
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
