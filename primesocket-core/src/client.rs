use pyo3::prelude::*;
use std::net::UdpSocket;

#[pyfunction]
pub fn send_message(ip: &str, port: u16, message: &str) -> PyResult<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    let server_addr = format!("{}:{}", ip, port);
    socket.send_to(message.as_bytes(), &server_addr)?;
    Ok(())
}
