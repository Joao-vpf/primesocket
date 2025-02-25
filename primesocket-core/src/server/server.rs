use super::request_handler::handler;
use super::server_state::ServerState;
use crate::utils::json::{Request, Response};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::cmp::min;
use std::sync::{Arc, RwLock};
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
/// * `start` - (Optional) The starting value of the number range to be processed.
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
/// primesocket_core.start_server(8080, start=0, end=1000, step=10)
/// ```
#[pyfunction(signature = (port, start=None, end=None, step=None))]
pub fn start_server(
    port: u16,
    start: Option<u64>,
    end: Option<u64>,
    step: Option<u64>,
) -> PyResult<()> {
    let start = start.unwrap_or(0);
    let end = match end {
        Some(e) => e,
        None => return Err(PyErr::new::<PyValueError, _>("Parameter 'end' is required")),
    };

    let step = step.unwrap_or(10);
    let step = min((end - start) * step / 100, 5000);

    // Start the server asynchronously
    let rt = Runtime::new().map_err(|e| {
        PyErr::new::<PyValueError, _>(format!("Failed to create Tokio runtime: {}", e))
    })?;

    rt.spawn(async move {
        if let Err(e) = run_server(port, start, end, step).await {
            eprintln!("❌ Server encountered an error while running: {:?}", e);
        }
    });

    println!("🚀 Server started on port {}", port);

    // Keeps the server running indefinitely
    loop {
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
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
async fn run_server(port: u16, start: u64, end: u64, step: u64) -> PyResult<()> {
    // Attempt to bind the UDP socket
    let socket = match UdpSocket::bind(format!("0.0.0.0:{}", port)).await {
        Ok(sock) => {
            println!("✅ Server listening on 0.0.0.0:{}", port);
            sock
        }
        Err(e) => {
            eprintln!("❌ Failed to bind UDP socket: {:?}", e);
            return Err(PyErr::new::<PyValueError, _>(format!(
                "Failed to bind UDP socket: {}",
                e
            )));
        }
    };

    // Shared state for managing prime number computation
    let server_state = Arc::new(RwLock::new(ServerState::new(start, end, step)));

    // Infinite loop to process incoming requests
    loop {
        let mut buffer = vec![0; 4096];

        match socket.recv_from(&mut buffer).await {
            Ok((size, src)) => {
                buffer.truncate(size);

                let request = String::from_utf8_lossy(&buffer[..size]);
                println!("📩 Received request from {}: {}", src, request);

                if let Some(request_data) = Request::from_json(&request) {
                    let response = handler(server_state.clone(), request_data).await;
                    println!("📤 Response being sent: {:?}", response);
                    socket
                        .send_to(response.to_json().as_bytes(), src)
                        .await
                        .unwrap();
                } else {
                    println!("⚠️ Invalid request format!");

                    let error_response = Response {
                        task: "error".to_string(),
                        status: "invalid_request".to_string(),
                        start: None,
                        end: None,
                        sieve: None,
                        last_checked: None,
                        primes: Some(Vec::new()),
                    };

                    socket
                        .send_to(error_response.to_json().as_bytes(), src)
                        .await
                        .unwrap();
                }
            }
            Err(e) => {
                eprintln!("❌ Failed to receive data: {:?}", e);
            }
        }
    }
}
