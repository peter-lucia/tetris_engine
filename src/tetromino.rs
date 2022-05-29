

// Useful:
// https://www.youtube.com/watch?v=8OK8_tHeCIA

/*
Every tetromino will be made from a 4x4 grid
 */

pub struct Tetromino {
    pub area: [[i32; 4]; 4],
    pub x: usize,
    pub y: usize,
}

impl Tetromino {
    pub fn rotate(&mut self) -> () {
        rotate(self);
    }
}

impl Default for Tetromino {
    fn default() -> Tetromino {
        Tetromino {
            area: [[0,0,1,0],
            [0,0,1,0],
            [0,0,1,0],
            [0,0,1,0]],
            x: 0,
            y: 6,
        }
    }
}

/*
Rotate 90 degrees clockwise
 */
fn rotate(t: &mut Tetromino) -> () {
    let n = t.area.len();
    let m = t.area[0].len();

    // transpose across left to right diagonal
    for i in 0..n {
        for j in i..m {
            let tmp = t.area[i][j];
            t.area[i][j] = t.area[j][i];
            t.area[j][i] = tmp;
        }
    }
    // reverse each row
    // same as a flip w/ respect to middle column
    for i in 0..n {
        t.area[i].reverse();
    }
}

/*
Rotate 90 degrees clockwise alternative implementation,
same concept
 */
fn rotate_alt(t: &mut Tetromino) -> () {
    let n = t.area.len();
    let m = t.area[0].len();

    // first rotation
    // with respect to main diagonal
    for i in 0..n {
        for j in i..m {
            let tmp = t.area[i][j];
            t.area[i][j] = t.area[j][i];
            t.area[j][i] = tmp;
        }
    }
    // Second rotation
    // with respect to middle column
    for i in 0..n {
        for j in 0..n/2 {
            let tmp = t.area[i][j];
            t.area[i][j] = t.area[i][n-j-1];
            t.area[i][n-j-1] = tmp;
        }
    }
}

pub trait TetrominoStraight {

    fn make_straight() -> Self;

}

impl TetrominoStraight for Tetromino {

    fn make_straight() -> Tetromino {
        return Tetromino {
            area:
            [[0,0,1,0],
             [0,0,1,0],
             [0,0,1,0],
             [0,0,1,0]],
            ..Default::default()
        }
    }

}

pub trait TetrominoSquare {
    fn make_square() -> Self;
}

impl TetrominoSquare for Tetromino {
    fn make_square() -> Self {
        return Tetromino {
            area: [[0,0,0,0],
                  [0,1,1,0],
                  [0,1,1,0],
                  [0,0,0,0]],
            ..Default::default()
        }
    }
}

pub trait TetrominoT {
    fn make_t() -> Self;
}

impl TetrominoT for Tetromino {
    fn make_t() -> Self {
        return Tetromino {
            area:
            [[0,0,0,0],
             [1,1,1,0],
             [0,1,0,0],
             [0,0,0,0]],
            ..Default::default()
        }
    }

}

pub trait TetrominoL {
    fn make_l() -> Tetromino;
}

impl TetrominoL for Tetromino {
    fn make_l() -> Tetromino {
        return Tetromino {
            area:
            [[1,0,0,0],
             [1,0,0,0],
             [1,1,0,0],
             [0,0,0,0]],
            ..Default::default()
        }
    }
}

pub trait TetrominoSkew {
    fn make_skew() -> Self;
}

impl TetrominoSkew for Tetromino {
    fn make_skew() -> Self {
        return Tetromino {
            area:
            [[1,1,0,0],
             [0,1,1,0],
             [0,0,0,0],
             [0,0,0,0]],
            ..Default::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tetromino::{TetrominoL, Tetromino};

    #[test]
    fn test_rotate() -> () {
        let mut t = Tetromino::make_l();
        t.area =
            [[1,0,0,0],
             [1,0,0,0],
             [1,1,0,0],
             [1,1,0,0],
        ];
        t.rotate();
        let mut expected_result =
            [[1,1,1,1],
             [1,1,0,0],
             [0,0,0,0],
             [0,0,0,0]];
        assert_eq!(t.area, expected_result);
    }

    fn test_rotate_90_basic() {

        let mut t = Tetromino::make_l();
        t.area =
            [
                [1,0,0,0],
                [1,0,0,0],
                [1,1,0,0],
                [1,1,0,0],
            ];
        t.rotate();
        let mut expected_result =
            [
                [1,1,1,1],
                [1,1,0,0],
                [0,0,0,0],
                [0,0,0,0],
            ];
        assert_eq!(t.area, expected_result);
    }

    #[test]
    fn test_rotate_90_tetromino() -> () {
        let mut t = Tetromino::make_l();
        t.rotate();
        let mut expected_result =
            [
                [0,1,1,1],
                [0,1,0,0],
                [0,0,0,0],
                [0,0,0,0]
            ];
        assert_eq!(t.area, expected_result);
        t.rotate();
        expected_result =
            [
                [0,0,1,1],
                [0,0,0,1],
                [0,0,0,1],
                [0,0,0,0]
            ];
        assert_eq!(t.area, expected_result);
    }

    #[test]
    fn test_rotate_full() -> () {
        let mut t = Tetromino::make_l();
        t.area =
            [[1,2,3,4],
             [5,6,7,8],
             [9,10,11,12],
             [13,14,15,16]];
        t.rotate();
        let mut expected_result =
            [[13,9,5,1],
             [14,10,6,2],
             [15,11,7,3],
             [16,12,8,4]];
        assert_eq!(t.area, expected_result);
    }
}
