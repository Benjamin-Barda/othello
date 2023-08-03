mod bitboard;
mod game;
mod mcts;

use bitboard::get_initial_white_bitboard;
use pyo3::prelude::*;

#[pyfunction]
fn get_white_initial_bitboard() -> PyResult<u64> {
    Ok(get_initial_white_bitboard())
}

#[pyfunction]
fn ping() -> PyResult<String> {
    Ok(String::from("pong"))
}

#[pymodule]
fn othello(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ping, m)?)?;
    m.add_function(wrap_pyfunction!(get_white_initial_bitboard, m)?)?;
    m.add_class::<game::Othello>()?;
    Ok(())
}

struct Node {
    children: Vec<Node>,
}
