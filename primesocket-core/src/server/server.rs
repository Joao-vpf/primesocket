use super::response_handler::handler;
use super::server_state::ServerState;
use crate::utils::json::{Request, Response};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::io::ErrorKind;
use tokio::net::UdpSocket;
use tokio::runtime::Runtime;

/// Starts a UDP server for processing client requests.
///
/// This function initializes a server that listens on a specified port and
/// handles incoming UDP messages asynchronously. The server processes prime
/// number computations using a sieve method.
///
/// # Arguments
///
/// * `port` - The UDP port where the server will listen.
/// * `end` - The ending value of the number range to be processed (mandatory).
/// * `step` - (Optional) Defines the processing step size.
///
/// # Errors
///
/// This function returns a `PyValueError` if the `end` parameter is not provided.
///
/// # Example (Python)
///
/// ```python
/// import primesocket_core
/// primesocket_core.start_server(8080, end=1000)
/// ```
#[pyfunction(signature = (port, end=None, verbose=None))]
pub fn start_server(port: u16, end: Option<u32>, verbose: Option<u8>) -> PyResult<()> {
    let verbose = verbose.unwrap_or(0);
    let start = 2;
    let end = match end {
        Some(e) => e,
        None => return Err(PyErr::new::<PyValueError, _>("Parameter 'end' is required")),
    };

    // Start the server asynchronously
    let rt = Runtime::new().map_err(|e| {
        PyErr::new::<PyValueError, _>(format!("Failed to create Tokio runtime: {}", e))
    })?;

    rt.block_on(async move {
        if let Err(e) = run_server(port, start, end, verbose).await {
            if verbose > 0 {
                eprintln!("❌ Server encountered an error while running: {:?}", e);
            }
        }
    });

    Ok(())
}

/// Runs the UDP server and processes client requests.
///
/// This function binds a UDP socket to the given port and listens for incoming
/// messages. It processes requests using a shared `ServerState` and responds accordingly.
///
/// # Arguments
///
/// * `port` - The UDP port to bind the socket.
/// * `start` - The start of the number range.
/// * `end` - The end of the number range.
/// * `step` - The step size for processing.
///
/// # Errors
///
/// This function returns a `PyValueError` if it fails to bind the UDP socket.
///
/// # Behavior
///
/// * It runs in an infinite loop, receiving UDP packets from clients.
/// * It processes the received JSON request using `handler`.
/// * It sends a response back to the client based on the processed request.
async fn run_server(port: u16, start: u32, end: u32, verbose: u8) -> PyResult<()> {
    // Attempt to bind the UDP socket
    let socket = match UdpSocket::bind(format!("0.0.0.0:{}", port)).await {
        Ok(sock) => {
            if verbose > 0 {
                println!("🚀 Server started on port {}", port);
            }
            sock
        }
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

    // Shared state for managing prime number computation
    let mut server_state = ServerState::new(start, end);
    let mut countdown = 0;

    loop {
        if server_state.status == "completed" {
            if countdown == 1 {
                let e = server_state.save_primes_to_file();
                if verbose > 0 && e.is_err() {
                    eprintln!("❌ Failed to save primes: {:?}", e);
                }
                break;
            }

            if countdown == 0 {
                countdown += 1;
            }
        }

        let mut buffer = vec![0; 65535];

        match socket.recv_from(&mut buffer).await {
            Ok((size, src)) => {
                buffer.truncate(size);
                let request = String::from_utf8_lossy(&buffer[..size]);

                if verbose > 1 {
                    println!("📩 Received request from {}: {}", src, request);
                }

                if let Some(request_data) = Request::from_json(&request) {
                    let response = handler(&mut server_state, request_data);
                    if verbose > 1 {
                        println!("📤 Response being sent: {:?}", response);
                    }
                    socket
                        .send_to(response.to_json().as_bytes(), src)
                        .await
                        .unwrap();
                } else {
                    if verbose > 1 {
                        println!("⚠️ Invalid request format!");
                    }

                    let error_response = Response {
                        task: "error".to_string(),
                        status: "invalid_request".to_string(),
                        start: None,
                        end: None,
                        primes: None,
                    };

                    socket
                        .send_to(error_response.to_json().as_bytes(), src)
                        .await
                        .unwrap();
                }
            }
            Err(e) => {
                if e.kind() == ErrorKind::ConnectionReset {
                    if verbose > 1 {
                        eprintln!("⚠️ Connection was reset by a remote client. Ignoring...");
                    }
                    continue; // Ignora e continua o loop
                } else {
                    if verbose > 0 {
                        eprintln!("❌ Failed to receive data: {:?}", e);
                    }
                }
            }
        }
    }

    Ok(())
}
