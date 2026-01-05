//! # Chapter 1: Permutations — Random Generation (Fisher–Yates)
//!
//! Randomly permutes a sequence in-place with uniform distribution.
//!
//! ## Algorithm Description
//!
//! - Iterate from the end of the array down to the second element.
//! - Pick a random index `j` in `0..=i` and swap elements at `i` and `j`.
//!
//! ## Complexity
//!
//! - Time: O(n)
//! - Space: O(1) (in-place)

use rand::Rng;

/// Shuffle elements of a mutable slice in-place using Fisher–Yates.
///
/// # Arguments
///
/// * `arr` — The mutable slice to shuffle.
///
/// # Example
///
/// ```
/// use rusty_combinatorial::permutations::random::fisher_yates;
/// let mut v = vec![1, 2, 3, 4];
/// fisher_yates(&mut v);
/// assert_eq!(v.len(), 4);
/// ```
pub fn fisher_yates<T>(arr: &mut [T]) {
    let mut rng = rand::thread_rng();
    for i in (1..arr.len()).rev() {
        let j = rng.gen_range(0..=i);
        arr.swap(i, j);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fisher_yates_length() {
        let mut v = vec![1, 2, 3, 4, 5];
        fisher_yates(&mut v);
        assert_eq!(v.len(), 5);
    }
}
