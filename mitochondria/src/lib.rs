//! This crate exposes some types with pyhton bindings, which can be reused

use pyo3::{pyclass, PyResult, pymodule, Python};
use pyo3::types::PyModule;

#[pyclass]
pub struct MitochondrialDna {
    pub ring_buffer: String,
}

#[pymodule]
fn mitochondria(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<MitochondrialDna>()?;
    Ok(())
}