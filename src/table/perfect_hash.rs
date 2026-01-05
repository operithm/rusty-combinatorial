//! # Utility: Tables — Perfect Hash
//!
//! Build a minimal perfect hash for a small set of keys.
//!
//! ## Complexity
//!
//! - Time & Space: O(n)

use std::collections::HashMap;

/// Build a simple perfect hash: map each key to its index.
///
/// # Arguments
///
/// * `keys` — Slice of string slices
///
/// # Returns
///
/// A `HashMap<&str, usize>` mapping each key to a unique index.

pub fn perfect_hash<'a>(keys: &'a [&'a str]) -> HashMap<&'a str, usize> {
    keys.iter().enumerate().map(|(i,&k)| (k, i)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_hash() {
        let keys = ["apple", "banana"];
        let h = perfect_hash(&keys);
        assert_eq!(h["apple"], 0);
        assert_eq!(h["banana"], 1);
    }
}
