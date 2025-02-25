use std::cmp::max;
use crate::utils::json::{Request, Response};
use crate::server::server_state::ServerState;

/// Handles incoming requests and processes them based on the requested task.
///
/// This function receives a request from the client, updates the server state,
/// and returns an appropriate response.
///
/// # Arguments
///
/// * `server_state` - A mutable reference to the server state.
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
/// - `"save"`: Updates the state with the latest processed number and primes.
/// - `"fetch"`: Returns the current list of identified prime numbers.
/// - Any other task: Returns an error response.
pub fn handler(server_state: &mut ServerState, request: Request) -> Response {
    // If the computation is completed, return the final result.
    if server_state.status == "completed" {
        return Response {
            task: "done".to_string(),
            status: server_state.status.clone(),
            start: None,
            end: None,
            primes: None,
        };
    }

    match request.task.as_str() {
        "start" => Response {
            task: "range".to_string(),
            status: server_state.status.clone(),
            start: Some(server_state.last_checked),
            end: Some(server_state.last_checked + server_state.step),
            primes: Some(server_state.primes.iter().take(10_000).cloned().collect::<Vec<u32>>()),
        },
        "save" => {
            let last_checked = request.end.unwrap_or(0);
            server_state.primes.extend(request.primes.unwrap_or([].to_vec()));
            server_state.last_checked = max(last_checked, server_state.last_checked);

            // If the last checked number reaches the end, mark as completed.
            if server_state.last_checked >= server_state.end {
                server_state.status = "completed".to_string();
                return Response {
                    task: "done".to_string(),
                    status: server_state.status.clone(),
                    start: None,
                    end: None,
                    primes: None,
                };
            }

            Response {
                task: "continue".to_string(),
                status: server_state.status.clone(),
                start: None,
                end: None,
                primes: None,
            }
        },
        _ => Response {
            task: "error".to_string(),
            status: "invalid_task".to_string(),
            start: None,
            end: None,
            primes: None,
        },
    }
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    /// Tests the `handler` function when a "start" request is sent.
    ///
    /// This test ensures that when a client sends a `"start"` request, the handler:
    /// - Returns a response with `"task": "range"`.
    /// - Has a `"status"` of `"processing"`.
    /// - Contains valid `start` and `end` values.
    #[test]
    fn test_handler_start_request() {
        let start = 0;
        let end = 100;
        let step = 10;

        let mut server_state = ServerState::new(start, end, step);

        let request = Request {
            task: "start".to_string(),
            end: None,
            primes: None,
        };

        let response = handler(&mut server_state, request);
        println!("Handler response: {:?}", response);

        assert_eq!(response.task, "range");
        assert_eq!(response.status, "processing");
        assert!(response.start.is_some());
        assert!(response.end.is_some());
    }
}
