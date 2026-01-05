//! # Chapter 7: Set Partitions — Generation
//!
//! Enumerate all ways to partition the set `{0,1,…,n-1}` into nonempty subsets.
//!
//! ## Algorithm Description
//!
//! - Recursively assign each element either to an existing block or start a new one.
//! - Backtrack once all elements are placed to collect each partition.
//!
//! ## Complexity
//!
//! - Time: O(B(n)) where B(n) is the Bell number
//! - Space: O(n) for recursion depth

/// Generate all set partitions of the set `{0,1,…,n-1}`.
///
/// # Arguments
///
/// * `n` — Number of elements in the set
///
/// # Returns
///
/// A `Vec` of partitions, each represented as a `Vec<Vec<usize>>` of blocks.
///
/// # Example
///
/// ```
/// use rusty_combinatorial::partition_set::generate_set::generate_set_partitions;
/// let parts = generate_set_partitions(3);
/// assert_eq!(parts.len(), 5); // B(3) = 5
/// ```
pub fn generate_set_partitions(n: usize) -> Vec<Vec<Vec<usize>>> {
    fn helper(
        i: usize,
        parts: &mut Vec<Vec<usize>>,
        out: &mut Vec<Vec<Vec<usize>>>,
        n: usize,
    ) {
        if i == n {
            out.push(parts.clone());
            return;
        }
        // Try adding to existing blocks
        for block in 0..parts.len() {
            parts[block].push(i);
            helper(i + 1, parts, out, n);
            parts[block].pop();
        }
        // Or create a new block
        parts.push(vec![i]);
        helper(i + 1, parts, out, n);
        parts.pop();
    }

    let mut result = Vec::new();
    helper(0, &mut Vec::new(), &mut result, n);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_set_partitions() {
        let p = generate_set_partitions(3);
        // Expect partitions of 3: 5 total
        assert_eq!(p.len(), 5);
        // Example: [{0,1,2}], [{0},{1,2}], [{1},{0,2}], ...
    }
}
