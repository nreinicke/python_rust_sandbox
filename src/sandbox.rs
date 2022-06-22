use pyo3::{
    exceptions::{PyIndexError, PyNotImplementedError},
    prelude::*,
};

#[pyclass]
pub struct ArrayWrap(Vec<usize>);

#[pymethods]
impl ArrayWrap {
    #[new]
    pub fn new(vec: Vec<usize>) -> Self {
        ArrayWrap(vec)
    }

    pub fn __len__(&self) -> usize {
        self.0.len()
    }

    pub fn __getitem__(&self, idx: usize) -> PyResult<usize> {
        let value = self.0.get(idx);
        match value {
            Some(value) => Ok(value.clone()),
            None => Err(PyIndexError::new_err("Index out of range")),
        }
    }
    pub fn __setitem__(&mut self, _idx: usize, _new_value: usize) -> PyResult<()> {
        Err(PyNotImplementedError::new_err(
            "Setting array value at index is not implemented. Set entire array.",
        ))
    }

    pub fn to_list(&self) -> PyResult<Vec<usize>> {
        Ok(self.0.clone())
    }
}
