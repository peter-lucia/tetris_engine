mod tetromino;
mod well;

use std::borrow::{Borrow, BorrowMut};
use well::{Tetris};
use crate::well::{Direction, Well};
use pyo3::{prelude::*, wrap_pyfunction};

#[cfg(feature = "python-lib")]
#[pyfunction]
pub fn create_game() -> Well {
    let mut _well: Well = Tetris::new();
    return _well;
}


/// So in python we can do: from rust_tetris import get_well
/// Example code: https://pyo3.rs/v0.14.5/module.html
/// 'static lifetimes live the duration of the program
/// Lifetime Sources:
/// https://doc.rust-lang.org/reference/lifetime-elision.html#lifetime-elision-in-functions
/// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html

#[cfg(feature = "python-lib")]
#[pymodule]
#[pyo3(name = "rust_tetris")]
fn setup_tetris(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_game, m)?)?;
    Ok(())
}