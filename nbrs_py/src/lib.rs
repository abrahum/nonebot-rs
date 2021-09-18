use pyo3::prelude::*;
use pyo3::types::PyDict;

pub struct PyEvent;

impl PyEvent {
    pub fn new(py: Python) -> PyObject {
        PyDict::new(py).to_object(py)
    }
}
