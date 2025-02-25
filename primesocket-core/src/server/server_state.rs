use std::sync::{Arc, RwLock};

/// Represents the state of the server for prime number computations.
///
/// The `ServerState` struct maintains the current range of numbers being processed,
/// the status of the computation, and intermediate results using a sieve method.
///
/// # Fields
///
/// * `start` - The starting number of the range to be processed.
/// * `end` - The ending number of the range to be processed.
/// * `step` - The step size used for processing the range.
/// * `last_checked` - The last number that has been processed.
/// * `sieve` - A vector representing the sieve used for prime number identification.
/// * `primes` - A list of identified prime numbers.
/// * `status` - The current status of the computation (e.g., `"processing"`, `"completed"`).
///
/// # Example
///
/// ```rust
/// let server_state = ServerState::new(0, 100, 10);
/// assert_eq!(server_state.start, 0);
/// assert_eq!(server_state.end, 100);
/// assert_eq!(server_state.status, "processing");
/// ```
#[derive(Clone, Debug)]
pub struct ServerState {
    pub start: u64,
    pub end: u64,
    pub step: u64,
    pub last_checked: u64,
    pub sieve: Vec<u8>,
    pub primes: Vec<u64>,
    pub status: String,
}

impl ServerState {
    /// Creates a new `ServerState` instance.
    ///
    /// This function initializes the server state with the given range and step size.
    /// The sieve is initialized as a vector of `1`s, representing all numbers as potentially prime.
    /// The `last_checked` value starts at `0`, and the computation status is set to `"processing"`.
    ///
    /// # Arguments
    ///
    /// * `start` - The starting number of the range.
    /// * `end` - The ending number of the range.
    /// * `step` - The step size for processing.
    ///
    /// # Returns
    ///
    /// A new instance of `ServerState` initialized with the given parameters.
    ///
    /// # Example
    ///
    /// ```rust
    /// let state = ServerState::new(0, 100, 10);
    /// assert_eq!(state.start, 0);
    /// assert_eq!(state.end, 100);
    /// assert_eq!(state.status, "processing");
    /// ```
    pub fn new(start: u64, end: u64, step: u64) -> ServerState {
        ServerState {
            start,
            end,
            step,
            last_checked: 0,
            sieve: vec![1; (end + 1) as usize],
            primes: Vec::new(),
            status: String::from("processing"),
        }
    }
}

/// A type alias for a shared, thread-safe `ServerState`.
///
/// The `SharedServerState` uses `Arc<RwLock<ServerState>>`, allowing multiple threads
/// to access and modify the server state safely.
pub type SharedServerState = Arc<RwLock<ServerState>>;

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the creation of a `ServerState` instance.
    ///
    /// This test ensures that:
    /// - The `start`, `end`, and `step` values are correctly assigned.
    /// - The `last_checked` starts at `0`.
    /// - The `sieve` is initialized with the correct size.
    /// - The initial status is `"processing"`.
    #[test]
    fn test_server_state_creation() {
        let start = 0;
        let end = 100;
        let step = 10;

        let server_state = ServerState::new(start, end, step);

        assert_eq!(server_state.start, 0);
        assert_eq!(server_state.end, 100);
        assert_eq!(server_state.step, 10);
        assert_eq!(server_state.last_checked, 0);
        assert_eq!(server_state.sieve.len(), (end + 1) as usize);
        assert_eq!(server_state.status, "processing");
    }
}
