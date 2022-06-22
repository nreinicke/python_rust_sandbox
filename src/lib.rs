use pyo3::prelude::*;

pub mod sandbox;
use crate::sandbox::ArrayWrap;

#[pymodule]
fn python_rust_sandbox(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ArrayWrap>()?;
    Ok(())
}
