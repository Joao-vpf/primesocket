use super::request_handler::{handler, send_request};
use crate::utils;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use tokio::net::UdpSocket;
use tokio::time::{timeout, Duration};
use utils::json::{Request, Response};

/// Starts a UDP client that sends requests to the server and handles the response.
///
/// This function initializes a UDP client that communicates with a server at a specified IP and port.
/// It sends computation requests and processes the server's responses accordingly.
///
/// # Arguments
///
/// * `ip` - The IP address of the server (e.g., "127.0.0.1").
/// * `port` - The UDP port where the server is listening.
/// * `verbose` - Optional verbosity level for logging output.
/// * `timeout_seconds` - Optional timeout in seconds for receiving responses.
///
/// # Errors
///
/// Returns a `PyValueError` if the client fails to initialize, send a request, or receive a response.
///
/// # Example (Python)
///
/// ```python
/// import primesocket_core
/// primesocket_core.start_client("127.0.0.1", 8080)
/// ```
#[pyfunction(signature = (ip, port, verbose=None, timeout_seconds=None))]
pub fn start_client(ip: &str, port: u16, verbose: Option<u8>, timeout_seconds: Option<u64>) -> PyResult<()> {
    let verbose = verbose.unwrap_or(0);
    let timeout_seconds = timeout_seconds.unwrap_or(120);
    
    // Create a new Tokio runtime to execute asynchronous operations
    let rt = tokio::runtime::Runtime::new().map_err(|e| {
        PyErr::new::<PyValueError, _>(format!("Failed to create Tokio runtime: {}", e))
    })?;

    // Run the client within the Tokio runtime
    rt.block_on(async {
        if let Err(e) = run_client(ip, port, verbose, timeout_seconds).await {
            if verbose > 0 {
                eprintln!("❌ Client encountered an error: {:?}", e);
            }
        }
    });

    Ok(())
}

/// Runs the UDP client that sends requests and handles server responses.
///
/// This function binds a UDP socket and repeatedly sends requests to the server.
/// It waits for responses and processes them accordingly.
///
/// # Arguments
///
/// * `ip` - The IP address of the server.
/// * `port` - The UDP port where the server is listening.
/// * `verbose` - Verbosity level for logging output.
/// * `timeout_seconds` - Timeout duration in seconds for receiving responses.
///
/// # Errors
///
/// Returns a `PyValueError` if the client fails to bind the socket, send a request, or process a response.
async fn run_client(ip: &str, port: u16, verbose: u8, timeout_seconds: u64) -> PyResult<()> {
    // Bind a UDP socket to any available port
    let socket = match UdpSocket::bind("0.0.0.0:0").await {
        Ok(sock) => sock,
        Err(e) => {
            if verbose > 0 {
                eprintln!("❌ Failed to bind UDP socket: {:?}", e);
            }
            return Err(PyErr::new::<PyValueError, _>(format!("Failed to bind UDP socket: {}", e)));
        }
    };

    loop {
        let request = Request {
            task: "start".to_string(),
            end: None,
            primes: None,
        };
        
        send_request(&socket, ip, port, &request, verbose).await?;

        let mut buffer = vec![0; 65535];
        
        match timeout(Duration::from_secs(timeout_seconds), socket.recv_from(&mut buffer)).await {
            Ok(Ok((size, src))) => {
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
            Ok(Err(e)) => {
                if verbose > 1 {
                    eprintln!("❌ Failed to receive data: {:?}", e);
                }
                return Err(PyErr::new::<PyValueError, _>(format!("Failed to receive response: {}", e)));
            }
            Err(_) => {
                if verbose > 0 {
                    eprintln!("⚠️ Connection lost: no response received within timeout. Disconnecting.");
                }
                break;
            }
        }
    }
    Ok(())
}
