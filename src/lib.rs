mod tetromino;
mod well;

use std::borrow::{Borrow, BorrowMut};
use well::{Tetris};
use crate::well::{Direction, Well};
#[cfg(all(feature = "python-lib", not(feature="wasm")))]
use pyo3::{prelude::*, wrap_pyfunction};
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(all(feature = "python-lib", not(feature="wasm")))]
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

#[cfg(all(feature = "python-lib", not(feature="wasm")))]
#[pymodule]
#[pyo3(name = "rust_tetris")]
fn setup_tetris(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_game, m)?)?;
    Ok(())
}

#[wasm_bindgen]
pub fn create_game() -> Well {
    let mut _well: Well = Tetris::new();
    return _well;
}