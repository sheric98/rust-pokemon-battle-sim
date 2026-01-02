use pyo3::prelude::*;
use pyo3::types::PyDict;

mod battle;
mod common;
mod core;
mod dex;
mod event;
mod query;

use battle::simulator::BattleSimulator;

#[pyclass]
struct Env {
    sim: BattleSimulator,
}

#[pymethods]
impl Env {
    #[new]
    fn new() -> Self {
        Self {
            sim: BattleSimulator::new(),
        }
    }
}

/// Helper: convert observation Rust struct → Python dict
fn serialize_obs(py: Python<'_>) -> PyObject {
    let dict = PyDict::new(py);
    dict.into()
}

#[pymodule]
fn pokemon_env(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Env>()?;
    Ok(())
}
