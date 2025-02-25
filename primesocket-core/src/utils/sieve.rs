use std::cmp::max;

/// Performs a segmented sieve to find prime numbers in a given range.
///
/// This function takes a starting number, an ending number, and a list of 
/// known primes and returns a vector containing the prime numbers within 
/// the range `[start, end]`. It uses a boolean vector to mark non-prime numbers.
///
/// # Arguments
///
/// * `start` - The starting number of the range (inclusive).
/// * `end` - The ending number of the range (inclusive).
/// * `primes` - A vector of prime numbers used to mark non-primes in the range.
///
/// # Returns
///
/// A `Vec<u32>` containing the prime numbers in the given range.
///
/// # Example
///
/// ```
/// let primes = vec![2, 3, 5, 7]; // Small primes to mark multiples
/// let result = sieve_segment(10, 30, primes);
/// assert_eq!(result, vec![11, 13, 17, 19, 23, 29]);
/// ```
pub fn sieve_segment(start: u32, end: u32, primes: Vec<u32>) -> Vec<u32> {
    let size = (end - start + 1) as usize;
    let mut is_prime = vec![1; size];  

    for &prime in &primes {
        if prime * prime > end {
            break;
        }

        let mut mul = max(prime * prime, start + (prime - start % prime) % prime);
        if mul == prime {
            mul += prime;
        }

        let prime = prime as usize;
        for j in (mul..=end).step_by(prime) {
            is_prime[(j - start) as usize] = 0;
        }
    }

    (start..=end)
        .filter(|&i| is_prime[(i - start) as usize] == 1)
        .collect::<Vec<u32>>()
}


#[cfg(test)]
mod tests {
    use super::*;

    /// Test the sieve_segment function with a known range and small primes.
    #[test]
    fn test_sieve_segment() {
        let primes = vec![2, 3, 5, 7];
        let result = sieve_segment(10, 30, primes);
        
        assert_eq!(result, vec![11, 13, 17, 19, 23, 29]);
    }

    /// Test sieve_segment with a range that contains no primes.
    #[test]
    fn test_sieve_segment_no_primes() {
        let primes = vec![2, 3, 5, 7];
        let result = sieve_segment(4, 8, primes);

        assert_eq!(result, vec![5, 7]);
    }

    /// Test sieve_segment with a small range containing only one prime.
    #[test]
    fn test_sieve_segment_single_prime() {
        let primes = vec![2, 3, 5, 7];
        let result = sieve_segment(17, 17, primes);

        assert_eq!(result, vec![17]);
    }

    /// Test sieve_segment with a range where the primes are already in the list.
    #[test]
    fn test_sieve_segment_with_known_primes() {
        let primes = vec![2, 3, 5, 7, 11, 13];
        let result = sieve_segment(10, 20, primes);

        assert_eq!(result, vec![11, 13, 17, 19]);
    }

    /// Test sieve_segment with an empty range.
    #[test]
    fn test_sieve_segment_empty_range() {
        let primes = vec![2, 3, 5, 7];
        let result = sieve_segment(30, 29, primes);

        assert_eq!(result, Vec::<u32>::new());
    }
}
