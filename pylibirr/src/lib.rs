use pyo3::prelude::*;

mod client;

/// A Python module implemented in Rust.
#[pymodule]
fn pylibirr(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(client::sum_as_string, m)?)?;
    m.add_class::<client::IRRClient>()?;

    Ok(())
}
