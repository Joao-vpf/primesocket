pub mod client;
pub mod server;
pub mod utils;

use crate::server::server::start_server;

use pyo3::prelude::*;

#[pymodule]
fn primesocket_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(start_server, m)?)?;
    Ok(())
}
