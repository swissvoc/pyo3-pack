#![feature(specialization)]

use pyo3::prelude::*;

#[pyclass]
struct DummyClass {}

#[pymethods]
impl DummyClass {
    #[staticmethod]
    fn get_42() -> PyResult<usize> {
        Ok(42)
    }
}

#[pymodule]
fn get_fourtytwo(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<DummyClass>().unwrap();
    m.add("fourtytwo", 42).unwrap();

    Ok(())
}
