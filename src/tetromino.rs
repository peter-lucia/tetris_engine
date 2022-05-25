use crate::board::Direction;

pub struct Tetromino {
    /*
    Coordinates
        0000
        0000
        0000
        0000
     */
    current_orientation: Direction,
    blocks_path: Vec<BlockVector>,
}

pub enum BlockVector {
    Forward,
    Left,
    Right
}

pub trait TetrominoStraight {

    fn new() -> Self;
    fn rotate() -> ();

}

impl TetrominoStraight for Tetromino {

    fn new() -> Self {
        return TetrominoStraight {
            current_orientation: Direction::Up,
            blocks_path: [
                BlockVector::Forward,
                BlockVector::Forward,
                BlockVector::Forward,
                BlockVector::Forward
            ],
        }
    }

    fn rotate() -> () {

    }

}

pub trait TetrominoSquare {

}

impl TetrominoSquare for Tetromino {

}

pub trait TetrominoT {

}

impl TetrominoT for Tetromino {

}

pub trait TetrominoL {

}

impl TetrominoL for Tetromino {

}

pub trait TetrominoSkew {

}

impl TetrominoSkew for Tetromino {

}
