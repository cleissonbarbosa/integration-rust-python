use pyo3::prelude::*;

/// Função que será chamada a partir do Python
#[pyfunction]
#[pyo3(signature = (a, b))]
fn sum_as_string(a: i64, b: i64) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// Módulo Python
#[pymodule]
fn my_rust_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}