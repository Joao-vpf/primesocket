use super::response_handler::handler;
use super::server_state::ServerState;
use crate::utils::json::{Request, Response};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::collections::HashSet;
use std::io::ErrorKind;
use std::sync::Arc;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::runtime::Builder;
use tokio::sync::{mpsc, Mutex};
use tokio::time::sleep;

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
/// * `verbose` - (Optional) Verbosity level for logging.
///
/// # Errors
///
/// This function returns a `PyValueError` if the `end` parameter is not provided.
#[pyfunction(signature = (port, end=None, verbose=None))]
pub fn start_server(port: u16, end: Option<u32>, verbose: Option<u8>) -> PyResult<()> {
    let verbose = verbose.unwrap_or(0);
    let start = 2;
    let end = match end {
        Some(e) => e,
        None => return Err(PyErr::new::<PyValueError, _>("Parameter 'end' is required")),
    };

    // Create a multi-threaded runtime
    let rt = Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|e| {
            PyErr::new::<PyValueError, _>(format!("Failed to create Tokio runtime: {}", e))
        })?;

    rt.block_on(async move {
        if let Err(e) = run_server(port, start, end, verbose).await {
            if verbose > 0 {
                eprintln!("❌ Server encountered an error: {:?}", e);
            }
        }
    });

    Ok(())
}

/// Runs the UDP server and processes client requests.
///
/// This function binds a UDP socket to the given port and listens for incoming
/// messages. It processes requests using a shared `ServerState` and enqueues the responses
/// to a dedicated task for sending.
///
/// # Arguments
///
/// * `port` - The UDP port to bind the socket.
/// * `start` - The start of the number range.
/// * `end` - The end of the number range.
/// * `verbose` - Verbosity level for logging.
///
/// # Errors
///
/// This function returns a `PyValueError` if it fails to bind the UDP socket.
async fn run_server(port: u16, start: u32, end: u32, verbose: u8) -> PyResult<()> {
    // Bind the UDP socket and wrap it in an Arc for thread-safe sharing
    let socket = match UdpSocket::bind(format!("0.0.0.0:{}", port)).await {
        Ok(sock) => {
            if verbose > 0 {
                println!("🚀 Server started on port {}", port);
            }
            Arc::new(sock)
        }
        Err(e) => {
            return Err(PyErr::new::<PyValueError, _>(format!(
                "Failed to bind UDP socket: {}",
                e
            )));
        }
    };

    let (response_tx, mut response_rx) = mpsc::channel::<(String, std::net::SocketAddr)>(100);

    let socket_for_sender = socket.clone();
    tokio::spawn(async move {
        while let Some((response_json, addr)) = response_rx.recv().await {
            if let Err(e) = socket_for_sender
                .send_to(response_json.as_bytes(), addr)
                .await
            {
                eprintln!("❌ Error sending response to {}: {:?}", addr, e);
            }
        }
    });

    let server_state = Arc::new(Mutex::new(ServerState::new(start, end)));
    let clients: Arc<Mutex<HashSet<String>>> = Arc::new(Mutex::new(HashSet::new()));

    loop {
        {
            let state = server_state.lock().await;
            if state.status == "completed" {
                if verbose > 0 {
                    println!("✅ Computation finished. Shutting down server...");
                }
                break;
            }
        }

        let mut buffer = vec![0; 65535];

        tokio::select! {
            result = socket.recv_from(&mut buffer) => {
                match result {
                    Ok((size, src)) => {
                        buffer.truncate(size);
                        let request = String::from_utf8_lossy(&buffer[..size]).to_string();
                        let client_addr = src.to_string();

                        {
                            let mut clients_lock = clients.lock().await;
                            if verbose > 0 && !clients_lock.contains(&client_addr) {
                                clients_lock.insert(client_addr.clone());
                                println!("🔗 New client connected: {}", client_addr);
                            }
                        }

                        let response_tx_clone = response_tx.clone();
                        let server_state_clone = server_state.clone();
                        let src_clone = src;

                        tokio::spawn(async move {
                            let response_json = {
                                let mut state = server_state_clone.lock().await;
                                if state.status == "completed" {
                                    if verbose > 0 {
                                        println!("✅ Computation finished. Saving results...");
                                    }
                                    if let Err(e) = state.save_primes_to_file() {
                                        eprintln!("❌ Error saving primes: {:?}", e);
                                    }
                                    return;
                                }
                                if let Some(request_data) = Request::from_json(&request) {
                                    let response = handler(&mut state, request_data);
                                    response.to_json()
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
                                    error_response.to_json()
                                }
                            };
                            if verbose > 1 {
                                println!("📤 Response being enqueued: {:?}", response_json);
                            }
                            if let Err(e) = response_tx_clone.send((response_json, src_clone)).await {
                                eprintln!("❌ Failed to enqueue response: {:?}", e);
                            }
                        });
                    }
                    Err(e) => {
                        if e.kind() == ErrorKind::ConnectionReset {
                            if verbose > 1 {
                                eprintln!("⚠️ Connection reset by peer. Ignoring...");
                            }
                            continue;
                        } else {
                            if verbose > 0 {
                                eprintln!("❌ Failed to receive data: {:?}", e);
                            }
                        }
                    }
                }
            },
            _ = sleep(Duration::from_millis(10)) => {
                continue;
            }
        }
    }
    Ok(())
}
