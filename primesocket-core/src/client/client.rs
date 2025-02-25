use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use tokio::net::UdpSocket;
use crate::utils;
use utils::json::{Request, Response};
use super::request_handler::{handler, send_request};

/// Starts a UDP client that sends requests to the server and handles the response.
/// 
/// This function allows the client to send requests to a server listening on a specific IP and port.
/// It sends requests with a defined range for prime number computation and processes the server's response.
///
/// # Arguments
///
/// * `ip` - The IP address of the server (e.g., "127.0.0.1").
/// * `port` - The UDP port where the server is listening.
///
/// # Errors
///
/// This function returns a `PyValueError` if the client fails to connect to the server or sends an invalid request.
///
/// # Example (Python)
///
/// ```python
/// import primesocket_core
/// primesocket_core.start_client("127.0.0.1", 8080)
/// ```
#[pyfunction(signature = (ip, port, verbose=None))]
pub fn start_client(ip: &str, port: u16, verbose: Option<u8>) -> PyResult<()> {
    let verbose = verbose.unwrap_or(0);
    // Set up the client asynchronously
    let rt = tokio::runtime::Runtime::new().map_err(|e| {
        PyErr::new::<PyValueError, _>(format!("Failed to create Tokio runtime: {}", e))
    })?;

    rt.block_on(async {
        if let Err(e) = run_client(ip, port, verbose).await {
            if verbose > 0 {
                eprintln!("❌ Client encountered an error: {:?}", e);
            }
        }
    });

    Ok(())
}

/// Runs the UDP client that sends requests and handles server responses.
/// 
/// This function binds a UDP socket and sends a request to the server.
/// It processes the server's response asynchronously.
///
/// # Arguments
///
/// * `ip` - The IP address of the server.
/// * `port` - The UDP port where the server is listening.
///
/// # Errors
///
/// Returns a `PyValueError` if it fails to communicate with the server.
async fn run_client(ip: &str, port: u16, verbose: u8) -> PyResult<()> {
    let socket = match UdpSocket::bind("0.0.0.0:0").await {
        Ok(sock) => sock,
        Err(e) => {
            if verbose > 0 {
                eprintln!("❌ Failed to bind UDP socket: {:?}", e);
            }
            return Err(PyErr::new::<PyValueError, _>(format!(
                "Failed to bind UDP socket: {}",
                e
            )));
        }
    };

    loop {
        let request = Request {
            task: "start".to_string(),
            end: None,
            primes: None,
        };    
        send_request(&socket, ip, port, &request, verbose).await?;

        // Receive the server's response
        let mut buffer = vec![0; 65535];
        match socket.recv_from(&mut buffer).await {
            Ok((size, src)) => {
                buffer.truncate(size);
                let response = String::from_utf8_lossy(&buffer);
                if verbose > 1 {
                    println!("📩 Received response from {}: {}", src, response);
                }
    
                if let Some(response_data) = Response::from_json(&response) {
                    if verbose > 1 {
                        println!("✅ Server Response: {:?}", response_data);
                    }
                    let request = handler(response_data).await;
                    match request.task.as_str() {
                        "save" => {
                            send_request(&socket, ip, port, &request, verbose).await?;
                            continue;
                        }
                        "continue" => {	
                            continue;
                        }
                        _ => {
                            if verbose > 1 {
                                eprintln!("✅ Client finished");
                            }
                            break;
                        }
                    }
                } else {
                    if verbose > 1 {
                        eprintln!("⚠️ Invalid response format!");
                    }
                    continue;
                }
            }
            Err(e) => {
                if verbose > 1 {
                    eprintln!("❌ Failed to receive data: {:?}", e);
                }
                return Err(PyErr::new::<PyValueError, _>(format!(
                    "Failed to receive response: {}",
                    e
                )));
            }
        }
    }
    Ok(())
}
