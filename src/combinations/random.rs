//! # Chapter 5: Combinations — Random Generation
//!
//! Generate a random k-combination from an n-element set using uniform sampling
//! without replacement.
//!
//! ## Algorithm Description
//!
//! Uses the `rand` crate’s index sampling to select k distinct indices
//! from 0..n, then sorts them.
//!
//! ## Complexity
//!
//! - Time: O(k) expected
//! - Space: O(k)

use rand::seq::index::sample;
use rand::thread_rng;

/// Returns a random k-combination from 0..n (sorted).
///
/// # Arguments
///
/// * `n` — Size of the base set
/// * `k` — Number of elements to choose
///
/// # Returns
///
/// A sorted `Vec<usize>` of length k containing unique indices.
///
/// # Example
///
/// ```
/// use rusty_combinatorial::combinations::random::random_combination;
/// let c = random_combination(10, 4);
/// assert_eq!(c.len(), 4);
/// assert!(c.windows(2).all(|w| w[0] < w[1]));
/// ```
pub fn random_combination(n: usize, k: usize) -> Vec<usize> {
    let mut rng = thread_rng();
    let mut c = sample(&mut rng, n, k).into_vec();
    c.sort_unstable();
    c
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_combination_bounds() {
        let c = random_combination(10, 4);
        assert_eq!(c.len(), 4);
        assert!(c.iter().all(|&x| x < 10));
    }
}
