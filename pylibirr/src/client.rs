use pyo3::prelude::*;
use pyo3::exceptions::PyConnectionError;
use libirr::irr::IRRClient as IRRBaseClient;

/// Formats the sum of two numbers as string.
#[pyfunction]
pub fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyclass]
pub struct IRRClient {
    client: IRRBaseClient,
}

#[pymethods]
impl IRRClient {
    #[new]
    pub fn new(py: Python) -> PyResult<Bound<PyAny>> {
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let client = IRRBaseClient::connect("whois.radb.net:43").await
                .map_err(|e| PyConnectionError::new_err(format!("Failed to connect: {}", e)))?;
            Ok(Self { client })
        })
    }

    pub async fn route(&mut self, asn: i32) -> PyResult<Vec<String>> {
        self.client.route(asn).await.map_err(|e| {
            PyConnectionError::new_err(format!("Failed to query: {}", e))
        })
    }
}