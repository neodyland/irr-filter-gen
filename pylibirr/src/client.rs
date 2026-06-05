use libirr::irr::IRRClient;
use pyo3::exceptions::PyConnectionError;
use pyo3::prelude::*;

#[pyfunction]
pub fn fetch_routes(py: Python, asn: i32) -> PyResult<Bound<PyAny>> {
    pyo3_async_runtimes::tokio::future_into_py(py, async move {
        let mut client = IRRClient::connect("whois.radb.net:43")
            .await
            .map_err(|e| PyConnectionError::new_err(format!("Failed to connect: {e}")))?;

        client
            .route(asn)
            .await
            .map_err(|e| PyConnectionError::new_err(format!("Failed to fetch routes: {e}")))
    })
}
