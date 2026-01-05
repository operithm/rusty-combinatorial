//! # Utility: Tables — Lookup Table
//!
//! Demonstrate safe lookup patterns in a precomputed table.
//!
//! ## Complexity
//!
//! - Time: O(1) per lookup
//! - Space: O(n)

/// Perform a safe lookup in a table by returning `Option<T>`.
///
/// # Arguments
///
/// * `table` — Slice of values
/// * `idx` — Index to retrieve
///
/// # Returns
///
/// Some(value) if `idx < table.len()`, else `None`.
pub fn lookup_table<T: Clone>(table: &[T], idx: usize) -> Option<T> {
    table.get(idx).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lookup_table() {
        let t = vec![10, 20, 30];
        assert_eq!(lookup_table(&t, 1), Some(20));
        assert_eq!(lookup_table(&t, 5), None);
    }
}
