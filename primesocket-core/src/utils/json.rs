use serde::{Deserialize, Serialize};

/// Represents a response from the server.
///
/// This struct contains information about the status of the request,
/// along with a range of numbers processed by the server. It can include
/// a sieve (a list of primes).
///
/// # Fields
///
/// * `task` - A string representing the type of task the server is performing.
/// * `status` - A string indicating the status of the task (e.g., "in_progress", "completed").
/// * `start` - The starting number in the range being processed (optional).
/// * `end` - The ending number in the range being processed (optional).
/// * `primes` - An optional vector containing the prime numbers identified so far.
///
/// # Example
///
/// ```
/// let response = Response {
///     task: "processing".to_string(),
///     status: "in_progress".to_string(),
///     start: Some(1),
///     end: Some(100),
///     primes: Some(vec![2, 3, 5, 7]),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub task: String,
    pub status: String,
    pub start: Option<u32>,
    pub end: Option<u32>,
    pub primes: Option<Vec<u32>>,
}

impl Response {
    /// Converts the `Response` struct into a JSON string.
    ///
    /// # Returns
    ///
    /// A `String` representing the serialized `Response` object in JSON format.
    ///
    /// # Example
    ///
    /// ```
    /// let response = Response {
    ///     task: "done".to_string(),
    ///     status: "success".to_string(),
    ///     start: Some(1),
    ///     end: Some(100),
    ///     primes: Some(vec![2, 3, 5, 7]),
    /// };
    /// let json = response.to_json();
    /// ```
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string())
    }

    /// Converts a JSON string into a `Response` struct.
    ///
    /// # Arguments
    ///
    /// * `json` - A JSON string to be deserialized into a `Response` object.
    ///
    /// # Returns
    ///
    /// An `Option<Response>`, which is `Some(Response)` if the deserialization was successful,
    /// or `None` if it failed.
    ///
    /// # Example
    ///
    /// ```
    /// let json = r#"{"task":"done", "status":"success", "start":1, "end":100, "primes":[2,3,5,7]}"#;
    /// let response = Response::from_json(json);
    /// ```
    pub fn from_json(json: &str) -> Option<Response> {
        serde_json::from_str(json).ok()
    }
}

/// Represents a request sent to the server.
///
/// This struct is used to send information to the server, such as the task
/// being requested and an optional sieve that may accompany the request.
///
/// # Fields
///
/// * `task` - A string representing the type of task the client wants the server to perform.
/// * `end` - An optional `u32` representing the end of the range for the task, if applicable.
/// * `primes` - An optional vector containing the prime numbers to be used for the task.
///
/// # Example
///
/// ```
/// let request = Request {
///     task: "start_process".to_string(),
///     end: Some(100),
///     primes: Some(vec![1, 0, 1, 1]),
/// };
/// ```
#[derive(Serialize, Deserialize, Debug)]
pub struct Request {
    pub task: String,
    pub end: Option<u32>,
    pub primes: Option<Vec<u32>>,
}

impl Request {
    /// Converts the `Request` struct into a JSON string.
    ///
    /// # Returns
    ///
    /// A `String` representing the serialized `Request` object in JSON format.
    ///
    /// # Example
    ///
    /// ```
    /// let request = Request {
    ///     task: "start_process".to_string(),
    ///     end: None,
    ///     primes: None,
    /// };
    /// let json = request.to_json();
    /// ```
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{}".to_string())
    }

    /// Converts a JSON string into a `Request` struct.
    ///
    /// # Arguments
    ///
    /// * `json` - A JSON string to be deserialized into a `Request` object.
    ///
    /// # Returns
    ///
    /// An `Option<Request>`, which is `Some(Request)` if the deserialization was successful,
    /// or `None` if it failed.
    ///
    /// # Example
    ///
    /// ```
    /// let json = r#"{"task":"start_process", "end":100, "primes":null}"#;
    /// let request = Request::from_json(json);
    /// ```
    pub fn from_json(json: &str) -> Option<Request> {
        serde_json::from_str(json).ok()
    }
}
