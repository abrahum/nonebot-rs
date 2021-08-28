use nonebot_rs as nbrs;
use pyo3::prelude::*;
use std::sync::{Arc, Mutex};

#[pyfunction]
fn run(py: Python) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async {
        let nb = nbrs::Nonebot::new();
        nb.pre_run();
        nbrs::axum_driver::run(Arc::new(Mutex::new(nb))).await;
        Ok(Python::with_gil(|py| py.None()))
    })
}

#[pymodule]
fn nbrs_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run, m)?)?;
    Ok(())
}
