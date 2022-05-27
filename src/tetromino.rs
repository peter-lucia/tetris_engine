use crate::well::{Direction, random_direction};

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
    shape: Vec<(i32, i32)>
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
            shape: vec![(0,0), (0,1), (0,2), (0,3)]
        }
    }

    fn rotate(&mut self) -> () {

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
            shape: vec![(0,0), (0,1), (1,0), (1,1)]
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
            shape: vec![(0,0), (0,1), (0,2), (1,1)]
        }
    }

}

pub trait TetrominoL {
    fn new() -> Self;
}

impl TetrominoL for Tetromino {
    fn new() -> Self {
        return Tetromino {
            current_orientation: random_direction(),
            shape: vec![(0,0), (1,0), (2,0), (2,1)]
        }
    }
}

pub trait TetrominoSkew {
    fn new() -> Self;
}

impl TetrominoSkew for Tetromino {
    fn new() -> Self {
        return Tetromino {
            current_orientation: random_direction(),
            shape: vec![(1,0), (1,1), (0,1), (0,2)]
        }
    }
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
