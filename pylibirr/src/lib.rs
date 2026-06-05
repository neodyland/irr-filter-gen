use pyo3::prelude::*;

mod client;

/// A Python module implemented in Rust.
#[pymodule]
fn pylibirr(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(client::fetch_routes, m)?)?;

    Ok(())
}
