use mitochondria::MitochondrialDna;
use pyo3::{pyclass, PyResult, pymodule, Python};
use pyo3::types::PyModule;

pub struct Chromosome {
    pub sequence: String,
}

#[pyclass]
pub struct CellDna {
    pub mitochondrial_dna: MitochondrialDna,
    pub nuclear_dna: Vec<Chromosome>,
}

#[pymodule]
fn cell(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_class::<CellDna>()?;
    Ok(())
}