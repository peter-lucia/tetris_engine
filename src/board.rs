use crate::tetromino::Tetromino;

pub struct Board {
    // https://docs.rs/ndarray/latest/ndarray/
}


pub enum Direction {
    Up,
    Down,
    Left,
    Right

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
