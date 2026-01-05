//! # Chapter 1: Permutations — Lexicographic Order
//!
//! This module implements the classic algorithm to generate the next permutation
//! of a sequence in lexicographic (dictionary) order.
//!
//! ## Algorithm Description
//!
//! Given a permutation `P`, this algorithm finds the next permutation `P'`
//! such that `P' > P` in lex order, or returns false if `P` is the last one.
//!
//! Steps:
//! 1. Scan from the right to find the longest non-increasing suffix.
//! 2. Identify the pivot — the element just before the suffix.
//! 3. Find the rightmost successor to the pivot in the suffix.
//! 4. Swap the pivot and successor, then reverse the suffix.
//!
//! ## Complexity
//!
//! - Time: O(n)
//! - Space: O(1) (in-place mutation)

/// Generate the next lexicographic permutation in-place.
///
/// # Arguments
///
/// * `v` - Mutable slice of elements to permute
///
/// # Returns
///
/// * `true` if the permutation was advanced to the next one
/// * `false` if it was already the last permutation
///
/// # Example
///
/// ```
/// use rusty_combinatorial::permutations::lex::next_lex_perm;
/// let mut v = vec![1, 2, 3];
/// assert!(next_lex_perm(&mut v));
/// assert_eq!(v, vec![1, 3, 2]);
/// ```
pub fn next_lex_perm<T: Ord>(v: &mut [T]) -> bool {
    if v.len() < 2 {
        return false;
    }
    // 1) Find pivot
    let mut i = v.len() - 2;
    while i != usize::MAX && v[i] >= v[i + 1] {
        if i == 0 { break; }
        i -= 1;
    }
    // If no pivot, we’re at last permutation
    if v[i] >= v[i + 1] {
        v.reverse();
        return false;
    }
    // 2) Find rightmost successor
    let mut j = v.len() - 1;
    while v[j] <= v[i] {
        j -= 1;
    }
    // 3) Swap and reverse suffix
    v.swap(i, j);
    v[i + 1..].reverse();
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_lex_perm_basic() {
        let mut v = vec![1, 2, 3];
        assert!(next_lex_perm(&mut v));
        assert_eq!(v, vec![1, 3, 2]);
    }

    #[test]
    fn test_last_permutation() {
        let mut v = vec![3, 2, 1];
        assert!(!next_lex_perm(&mut v));
        assert_eq!(v, vec![1, 2, 3]); // wrapped to first
    }
}
