use nonebot_rs as nbrs;
use pyo3::prelude::*;

#[pyfunction]
fn run(py: Python) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::local_future_into_py(py, async {
        let nb = nbrs::Nonebot::new();
        nb.async_run().await;
        Ok(Python::with_gil(|py| py.None()))
    })
}

// #[pymodule]
// fn nbrs_py(_py: Python, m: &PyModule) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(run, m)?)?;
//     Ok(())
// }
