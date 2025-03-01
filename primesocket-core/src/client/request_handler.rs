use crate::utils;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use tokio::net::UdpSocket;
use utils::json::{Request, Response};
use utils::sieve::sieve_segment;

/// Handles incoming requests and processes them based on the requested task.
///
/// This function processes different types of tasks:
/// - If the task is `"range"`, it computes a new range of prime numbers using the `sieve_segment` function.
/// - If the task is `"continue"`, it indicates that the server should continue processing.
/// - Any other task is handled with a `"close"` response.
///
/// # Arguments
///
/// * `response` - A `Response` object containing the task to be processed and optional parameters.
///
/// # Returns
///
/// A `Request` object containing the task to be processed next along with any relevant data.
pub async fn handler(response: Response) -> Request {
    match response.task.as_str() {
        "range" => {
            let start = response.start.unwrap();
            let end = response.end.unwrap();
            let primes = response.primes.unwrap();
            let result = sieve_segment(start, end, primes);
            Request {
                task: "save".to_string(),
                end: Some(end),
                primes: Some(result),
            }
        }
        "continue" => Request {
            task: "continue".to_string(),
            end: None,
            primes: None,
        },
        _ => Request {
            task: "close".to_string(),
            end: None,
            primes: None,
        },
    }
}

/// Sends a request to the specified UDP socket and target address.
///
/// This function serializes a `Request` into JSON format and sends it over the socket to the specified target address.
///
/// # Arguments
///
/// * `socket` - The `UdpSocket` to send the request through.
/// * `ip` - The target IP address to send the request to.
/// * `port` - The target port to send the request to.
/// * `request` - The `Request` to be sent.
/// * `verbose` - Verbosity level for logging output.
///
/// # Returns
///
/// This function returns a `PyResult<()>`, indicating success or failure. If the request fails to send,
/// an error is returned with a message describing the failure.
pub async fn send_request(
    socket: &UdpSocket,
    ip: &str,
    port: u16,
    request: &Request,
    verbose: u8,
) -> PyResult<()> {
    let request_json = request.to_json();
    if verbose > 1 {
        println!("📩 Sending request to {}:{}: {}", ip, port, request_json);
    }

    let target = format!("{}:{}", ip, port);

    socket
        .send_to(request_json.as_bytes(), &target)
        .await
        .map_err(|e| PyErr::new::<PyValueError, _>(format!("Failed to send request: {}", e)))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::json::Response;

    /// Tests the `handler` function when a "range" request is sent.
    ///
    /// This test ensures that when a client sends a `"range"` request:
    /// - The function computes a valid list of primes using `sieve_segment`.
    /// - The function returns a `"save"` task with the new primes and updated `end`.
    #[tokio::test]
    async fn test_handler_range_request() {
        let response = Response {
            task: "range".to_string(),
            status: "processing".to_string(),
            start: Some(0),
            end: Some(100),
            primes: Some(vec![2, 3, 5, 7, 11]),
        };

        let request = handler(response).await;
        assert_eq!(request.task, "save");
        assert_eq!(request.end, Some(100));
        assert!(request.primes.is_some());
    }

    /// Tests the `handler` function when a "continue" request is sent.
    ///
    /// This test ensures that when a client sends a `"continue"` request:
    /// - The function returns a `"continue"` task with no primes or end value.
    #[tokio::test]
    async fn test_handler_continue_request() {
        let response = Response {
            task: "continue".to_string(),
            status: "completed".to_string(),
            start: None,
            end: None,
            primes: None,
        };

        let request = handler(response).await;
        assert_eq!(request.task, "continue");
        assert!(request.end.is_none());
        assert!(request.primes.is_none());
    }
}
