use super::server_state::{ServerState, SharedServerState};
use crate::utils::json::{Request, Response};

/// Handles incoming requests and processes them based on the requested task.
///
/// This function receives a request from the client, updates the shared server state,
/// and returns an appropriate response.
///
/// # Arguments
///
/// * `server_state` - A shared and thread-safe reference (`RwLock`) to the server state.
/// * `request` - A `Request` object containing the task and optional parameters.
///
/// # Returns
///
/// A `Response` struct containing the task status, relevant data (if applicable),
/// and any computed results.
///
/// # Task Handling
///
/// - `"start"`: Returns the range of numbers to be processed.
/// - `"save"`: Updates the state with the latest processed number and sieve.
/// - `"fetch"`: Returns the current state of the sieve.
/// - Any other task: Returns an error response.
///
/// # Example
///
/// ```rust
/// let server_state = Arc::new(RwLock::new(ServerState::new(0, 100, 10)));
///
/// let request = Request {
///     task: "start".to_string(),
///     end: None,
///     sieve: None,
/// };
///
/// let response = handler(server_state.clone(), request).await;
/// assert_eq!(response.task, "range");
/// assert_eq!(response.status, "processing");
/// ```
pub async fn handler(server_state: SharedServerState, request: Request) -> Response {
    let mut state = server_state.write().unwrap();

    // If the computation is completed, return the final result.
    if state.status == "completed" {
        return Response {
            task: "done".to_string(),
            status: state.status.clone(),
            start: None,
            end: None,
            sieve: None,
            last_checked: None,
            primes: Some(state.primes.clone()),
        };
    }

    match request.task.as_str() {
        "start" => Response {
            task: "range".to_string(),
            status: state.status.clone(),
            start: Some(state.last_checked),
            end: Some(state.last_checked + state.step),
            sieve: Some(state.sieve.clone()),
            last_checked: Some(state.last_checked),
            primes: Some(state.primes.clone()),
        },
        "save" => {
            let last_checked = request.end.unwrap_or(0);
            if last_checked > state.last_checked {
                state.last_checked = last_checked;
                state.sieve = request.sieve.unwrap_or(state.sieve.clone());
            }

            // If the last checked number reaches the end, mark as completed.
            if state.last_checked >= state.end {
                state.status = "completed".to_string();
                return Response {
                    task: "done".to_string(),
                    status: state.status.clone(),
                    start: None,
                    end: None,
                    sieve: None,
                    last_checked: None,
                    primes: Some(state.primes.clone()),
                };
            }

            Response {
                task: "save".to_string(),
                status: state.status.clone(),
                start: Some(state.start),
                end: Some(state.end),
                sieve: Some(state.sieve.clone()),
                last_checked: Some(state.last_checked),
                primes: Some(state.primes.clone()),
            }
        }
        "fetch" => Response {
            task: "fetch".to_string(),
            status: state.status.clone(),
            start: None,
            end: None,
            sieve: Some(state.sieve.clone()),
            last_checked: None,
            primes: Some(state.primes.clone()),
        },
        _ => Response {
            task: "error".to_string(),
            status: "invalid_task".to_string(),
            start: None,
            end: None,
            sieve: None,
            last_checked: None,
            primes: Some(Vec::new()),
        },
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use std::sync::{Arc, RwLock};

    /// Tests the `handler` function when a "start" request is sent.
    ///
    /// This test ensures that when a client sends a `"start"` request, the handler:
    /// - Returns a response with `"task": "range"`.
    /// - Has a `"status"` of `"processing"`.
    /// - Contains valid `start` and `end` values.
    #[tokio::test]
    async fn test_handler_start_request() {
        let start = 0;
        let end = 100;
        let step = 10;

        let server_state = Arc::new(RwLock::new(ServerState::new(start, end, step)));

        let request = Request {
            task: "start".to_string(),
            end: None,
            sieve: None,
        };

        let response = handler(server_state.clone(), request).await;
        println!("Handler response: {:?}", response);

        assert_eq!(response.task, "range");
        assert_eq!(response.status, "processing");
        assert!(response.start.is_some());
        assert!(response.end.is_some());
    }
}
