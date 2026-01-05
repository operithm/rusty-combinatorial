//! # Chapter 7: Set Partitions — Fixed‐Block Generation
//!
//! Enumerate all ways to partition `{0,1,…,n-1}` into exactly `k` nonempty subsets.
//!
//! ## Algorithm Description
//!
//! - Backtracking similar to `generate_set_partitions` but only allow exactly `k` blocks.
//!
//! ## Complexity
//!
//! - Time: O(S(n,k)) where S(n,k) is Stirling number of second kind

/// Generate all set partitions of `n` elements into exactly `k` blocks.
///
/// # Arguments
///
/// * `n` — Number of elements
/// * `k` — Number of blocks
///
/// # Returns
///
/// A `Vec` of partitions, each a `Vec<Vec<usize>>` with `k` blocks.
///
/// # Example
///
/// ```
/// use rusty_combinatorial::partition_set::fixed_k::fixed_k_partitions;
/// let parts = fixed_k_partitions(4, 2);
/// for p in &parts {
///     assert_eq!(p.len(), 2);
/// }
/// ```
pub fn fixed_k_partitions(n: usize, k: usize) -> Vec<Vec<Vec<usize>>> {
    fn helper(
        i: usize,
        k: usize,
        parts: &mut Vec<Vec<usize>>,
        out: &mut Vec<Vec<Vec<usize>>>,
        n: usize,
    ) {
        if i == n {
            if parts.len() == k {
                out.push(parts.clone());
            }
            return;
        }
        for block in 0..parts.len() {
            parts[block].push(i);
            helper(i + 1, k, parts, out, n);
            parts[block].pop();
        }
        if parts.len() < k {
            parts.push(vec![i]);
            helper(i + 1, k, parts, out, n);
            parts.pop();
        }
    }

    let mut result = Vec::new();
    helper(0, k, &mut Vec::new(), &mut result, n);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_k_partitions() {
        let p = fixed_k_partitions(4, 2);
        assert!(p.len() > 0);
        for part in p {
            assert_eq!(part.len(), 2);
        }
    }
}
