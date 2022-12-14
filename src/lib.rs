mod tetromino;
mod well;

use std::borrow::{Borrow, BorrowMut};
use well::{Tetris};
use crate::well::{Direction, Well};
use pyo3::{prelude::*, wrap_pyfunction};

#[pyfunction]
pub fn create_game() -> Well {
    let mut _well: Well = Tetris::new();
    return _well;
}

#[pyfunction]
fn setup_game<'py>(py: Python<'py>, obj: PyObject) -> PyResult<Well> {
    let mut _well: Well = obj.extract::<Well>(py)?;
    _well.setup();
    return Ok(_well);
}

#[pyfunction]
fn run_frame<'py>(py: Python<'py>, obj: PyObject) -> PyResult<Well> {
    let mut _well: Well = obj.extract::<Well>(py)?;
    _well.run_frame();
    return Ok(_well);
}

#[pyfunction]
fn move_down<'py>(py: Python<'py>, obj: PyObject) -> PyResult<Well> {
    let mut _well: Well = obj.extract::<Well>(py)?;
    _well.move_tetromino(Direction::Down);
    return Ok(_well);
}

#[pyfunction]
fn quit<'py>(py: Python<'py>, obj: PyObject) -> PyResult<Well> {
    let mut _well: Well = obj.extract::<Well>(py)?;
    _well.quit();
    return Ok(_well);
}

/// So in python we can do: from rust_tetris import get_well
/// Example code: https://pyo3.rs/v0.14.5/module.html
/// 'static lifetimes live the duration of the program
/// Lifetime Sources:
/// https://doc.rust-lang.org/reference/lifetime-elision.html#lifetime-elision-in-functions
/// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html
#[pymodule]
#[pyo3(name = "rust_tetris")]
fn setup_tetris(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(create_game, m)?)?;
    m.add_function(wrap_pyfunction!(setup_game, m)?)?;
    m.add_function(wrap_pyfunction!(move_down, m)?)?;
    m.add_function(wrap_pyfunction!(run_frame, m)?)?;
    m.add_function(wrap_pyfunction!(quit, m)?)?;
    Ok(())
}