use std::fs::File;
use std::io::{self, Write};

/// Represents the server state for prime number computations.
///
/// The `ServerState` struct maintains the current range of numbers being processed,
/// the status of the computation, and intermediate results using a sieve method.
///
/// # Fields
///
/// * `end` - The upper limit of the number range to be processed.
/// * `step` - The step size used for processing the range.
/// * `last_checked` - The last number that has been processed.
/// * `primes` - A list of identified prime numbers.
/// * `status` - The current status of the computation (e.g., "processing", "completed").
///
/// # Example
///
/// ```rust
/// let server_state = ServerState::new(0, 100, 10);
/// assert_eq!(server_state.last_checked, 0);
/// assert_eq!(server_state.end, 100);
/// assert_eq!(server_state.status, "processing");
/// ```
#[derive(Clone, Debug)]
pub struct ServerState {
    pub end: u32,
    pub step: u32,
    pub last_checked: u32,
    pub primes: Vec<u32>,
    pub status: String,
}

impl ServerState {
    /// Creates a new instance of `ServerState`.
    ///
    /// This function initializes the server state with the given range and step size.
    /// The `last_checked` value starts at `start`, and the computation status is set to "processing".
    ///
    /// # Arguments
    ///
    /// * `start` - The starting number of the range.
    /// * `end` - The upper limit of the number range.
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
    /// assert_eq!(state.last_checked, 0);
    /// assert_eq!(state.end, 100);
    /// assert_eq!(state.status, "processing");
    /// ```
    pub fn new(start: u32, end: u32) -> ServerState {
        ServerState {
            end,
            step: 1000,
            last_checked: start,
            primes: {
                let mut primes = Vec::with_capacity(10000);
                primes.extend(vec![2, 3, 5, 7, 11 ,13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97]);
                primes
            },
            status: String::from("processing"),
        }
    }

    /// Saves the list of identified prime numbers to a file.
    ///
    /// This function writes the contents of `primes` into a file named `primes.txt`.
    /// Each prime number is written on a separate line.
    ///
    /// # Errors
    ///
    /// Returns an `io::Result<()>` indicating whether the file was successfully created and written.
    ///
    /// # Example
    ///
    /// ```rust
    /// let state = ServerState::new(0, 100, 10);
    /// state.save_primes_to_file().expect("Failed to save primes");
    /// ```
    pub fn save_primes_to_file(&self) -> io::Result<()> {
        let mut file = File::create("primes.txt")?;
        for prime in &self.primes {
            writeln!(file, "{}", prime)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the creation of a `ServerState` instance.
    ///
    /// This test ensures that:
    /// - The `end` and `step` values are correctly assigned.
    /// - The `last_checked` starts at `0`.
    /// - The `primes` vector is initialized with values.
    /// - The initial status is "processing".
    #[test]
    fn test_server_state_creation() {
        let start = 0;
        let end = 100;

        let server_state = ServerState::new(start, end);

        assert_eq!(server_state.last_checked, 0);
        assert_eq!(server_state.end, 100);
        assert_eq!(server_state.step, 5);
        assert!(!server_state.primes.is_empty());
        assert_eq!(server_state.status, "processing");
    }
}
