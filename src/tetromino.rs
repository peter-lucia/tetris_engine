use crate::board::{Direction, random_direction};

/*
These structs should be redesigned at least once.
 */
pub struct Tetromino {
    /*
    Coordinates
        0000
        0000
        0000
        0000
     */
    current_orientation: Direction,

}

pub enum BlockVector {
    Straight,
    Left,
    Right
}

pub trait TetrominoStraight {

    fn new() -> Self;
    fn rotate(&mut self) -> ();

}

impl TetrominoStraight for Tetromino {

    fn new() -> Tetromino {
        return Tetromino {
            current_orientation: random_direction(),
        }
    }

    fn rotate(&mut self) -> () {
        // self.current_orientation += 1;
    }

}

pub trait TetrominoSquare {

    fn new() -> Self;
    fn get_string(&self) -> String;
}

impl TetrominoSquare for Tetromino {
    fn new() -> Self {
        return Tetromino {
            current_orientation: random_direction(),
        }
    }

    fn get_string(&self) -> String {
        return "Testing".to_string();

    }
}

pub trait TetrominoT {

    fn new() -> Self;
}

impl TetrominoT for Tetromino {
    fn new() -> Self {
        return Tetromino {
            current_orientation: random_direction(),
        }
    }

}

pub trait TetrominoL {

}

impl TetrominoL for Tetromino {

}

pub trait TetrominoSkew {

}

impl TetrominoSkew for Tetromino {

}

#[cfg(test)]
mod tests {
    use crate::tetromino::TetrominoSquare;
    use crate::tetromino::Tetromino;

    #[test]
    fn test_my_atoi_1() {
        let tetromino: Tetromino = TetrominoSquare::new();
        let result = tetromino.get_string();
        println!("{}", result)
    }

}
