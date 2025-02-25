pub mod client;
pub mod server;
pub mod utils;

use crate::server::server::start_server;
use crate::client::client::start_client;

use pyo3::prelude::*;

#[pymodule]
fn primesocket_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(start_server, m)?)?;
    m.add_function(wrap_pyfunction!(start_client, m)?)?;
    Ok(())
}
