//! # Chapter 5: Combinations — Lexicographic Generation
//!
//! Enumerate all k-combinations of an n-element set in lex order.
//!
//! ## Algorithm Description
//!
//! - Represent a combination by a vector `a` of length k with increasing indices.
//! - To advance, scan from right to find an element that can be incremented.
//! - Increment it and reset all subsequent elements to follow in minimal order.
//!
//! ## Complexity
//!
//! - Time per combination: O(k)
//! - Space: O(k)

/// Advance the combination `a` to the next k-combination in lex order.
///
/// # Arguments
///
/// * `a` – Current combination indices (0..n-1), length k
/// * `n` – Total number of elements
///
/// # Returns
///
/// `true` if successfully advanced; `false` if already at last combination.
///
/// # Example
///
/// ```
///  use rusty_combinatorial::combinations::lex::next_combination;
/// let mut c = vec![0,1,2];
/// assert!(next_combination(&mut c, 5));
/// assert_eq!(c, vec![0,1,3]);
/// ```
pub fn next_combination(a: &mut Vec<usize>, n: usize) -> bool {
    let k = a.len();
    a.sort();
    for i in (0..k).rev() {
        if a[i] < n - k + i {
            a[i] += 1;
            for j in i+1..k {
                a[j] = a[j-1] + 1;
            }
            return true;
        }
    }
    false
}

/// Generate all k-combinations of 0..n.
///
/// # Arguments
///
/// * `n` – Size of the set
/// * `k` – Combination length
///
/// # Returns
///
/// A vector of all combinations as sorted index vectors.
///
/// # Example
///
/// ```
/// use rusty_combinatorial::combinations::lex::all_combinations;
/// let all = all_combinations(5, 3);
/// assert_eq!(all.len(), 10);
/// ```
pub fn all_combinations(n: usize, k: usize) -> Vec<Vec<usize>> {
    let mut a: Vec<usize> = (0..k).collect();
    let mut result = vec![a.clone()];
    while next_combination(&mut a, n) {
        result.push(a.clone());
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_combinations() {
        let combs = all_combinations(5, 3);
        assert_eq!(combs[0], vec![0,1,2]);
        assert_eq!(combs.last().unwrap(), &vec![2,3,4]);
    }

    #[test]
    fn test_next_combination() {
        let mut c = vec![0,1,2];
        assert!(next_combination(&mut c, 5));
        assert_eq!(c, vec![0,1,3]);
    }

    #[test]
    fn test_next_combination_full() {
        let mut c = vec![0,1,2,5,3,6];
        assert!(next_combination(&mut c, 7));
        assert_eq!(c, vec![0,1,2,4,5,6]);
    }
}
