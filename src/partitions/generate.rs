//! # Chapter 6: Integer Partitions — Generation
//!
//! Enumerate all integer partitions of a positive integer `n`.
//!
//! A partition is a sequence of positive integers that sum to `n`, listed in non‐increasing order.
//!
//! ## Algorithm Description
//!
//! - Recursively choose the next part ≤ the previous part (to maintain order).
//! - Backtrack and explore all possibilities until the sum reaches `n`.
//!
//! ## Complexity
//!
//! - Time: O(p(n)), where p(n) is the number of partitions of `n`
//! - Space: O(n) recursion depth

/// Generate all integer partitions of `n`.
///
/// # Arguments
///
/// * `n` — The integer to partition
///
/// # Returns
///
/// A `Vec<Vec<usize>>`, each inner vector is a non‐increasing partition summing to `n`.
///
/// # Example
///
/// ```
/// use rusty_combinatorial::partitions::generate::generate_partitions;
/// let parts = generate_partitions(4);
/// assert!(parts.contains(&vec![2,2]));
/// assert!(parts.contains(&vec![1,1,1,1]));
/// ```
pub fn generate_partitions(n: usize) -> Vec<Vec<usize>> {
    fn helper(
        rem: usize,
        max_part: usize,
        current: &mut Vec<usize>,
        result: &mut Vec<Vec<usize>>,
    ) {
        if rem == 0 {
            result.push(current.clone());
            return;
        }
        for part in (1..=max_part).rev() {
            if part <= rem {
                current.push(part);
                helper(rem - part, part, current, result);
                current.pop();
            }
        }
    }

    let mut result = vec![];
    helper(n, n, &mut Vec::new(), &mut result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_partitions_simple() {
        let parts = generate_partitions(4);
        assert!(parts.contains(&vec![4]));
        assert!(parts.contains(&vec![2, 2]));
        assert!(parts.contains(&vec![1, 1, 1, 1]));
        assert_eq!(parts.iter().map(|p| p.iter().sum::<usize>()).all(|s| s == 4), true);
    }
}
