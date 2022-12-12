mod tetromino;
mod well;

use std::borrow::{Borrow, BorrowMut};
use well::{Tetris};
use crate::well::{start_game, Well};
use pyo3::{prelude::*, wrap_pyfunction};

#[pyfunction]
pub fn MyTetris() -> Well {
    let mut well: Well = Tetris::new();
    return well;
}


#[pyfunction]
fn start<'p>(py: Python<'p>, obj: PyObject) -> PyResult<&'p PyAny> {
    // todo: don't return a copy, return ref to original
    let mut _well: Well = obj.extract::<Well>(py)?;
    pyo3_asyncio::async_std::future_into_py(py, async {
        // let mut _well: Well = Tetris::new();
        start_game(&mut _well).await;
        return Ok((_well));
    })
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
    m.add_function(wrap_pyfunction!(MyTetris, m)?)?;
    m.add_function(wrap_pyfunction!(start, m)?)?;
    Ok(())
}