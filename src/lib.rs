use pyo3::prelude::*;
use pyo3::types::PyDict;

mod battle;
mod core;
mod dex;
mod event;

use battle::simulator::BattleSimulator;

#[pyclass]
struct Env {
    sim: BattleSimulator,
}

#[pymethods]
impl Env {
    #[new]
    fn new() -> Self {
        Self { sim: BattleSimulator::new() }
    }

    /// Reset environment and return initial observation.
    fn reset(&mut self, py: Python<'_>) -> PyObject {
        let obs = self.sim.reset();
        serialize_obs(py)
    }

    /// Step the environment with an action.
    ///
    /// Returns (obs, reward, done, info)
    fn step(&mut self, py: Python<'_>, action: usize) -> PyObject {
        let (obs, reward, done, info) = self.sim.step(action);

        let dict = PyDict::new(py);
        dict.set_item("obs", serialize_obs(py)).unwrap();
        dict.set_item("reward", reward).unwrap();
        dict.set_item("done", done).unwrap();
        dict.set_item("info", info).unwrap();

        dict.into()
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
