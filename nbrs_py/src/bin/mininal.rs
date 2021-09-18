use nbrs_py;
use pyo3::prelude::*;

fn main() {
    Python::with_gil(|py| {
        py.eval("print('Hello, World')", None, None).unwrap();
    });
}
