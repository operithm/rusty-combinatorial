//! # Utility: Tables — Value Compression
//!
//! Compress an array of values into unique IDs (0..m-1).
//!
//! ## Complexity
//!
//! - Time: O(n log n) for sort/dedup
//! - Space: O(n)

use std::collections::HashMap;

/// Compress values by mapping each unique value to a compact integer ID.
///
/// # Arguments
///
/// * `data` — Slice of values that implement `Ord + Copy`
///
/// # Returns
///
/// Tuple `(encoded, unique_values)`:
/// - `encoded`: `Vec<usize>` IDs
/// - `unique_values`: sorted unique values
use std::hash::Hash;

pub fn compress<T: Ord + Copy + Hash>(data: &[T]) -> (Vec<usize>, Vec<T>) {
    let mut uniq = data.to_vec();
    uniq.sort();
    uniq.dedup();
    let map: HashMap<T, usize> = uniq.iter().cloned().enumerate().map(|(i,v)| (v, i)).collect();
    let encoded = data.iter().map(|&x| map[&x]).collect();
    (encoded, uniq)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress() {
        let data = vec![3,1,3,2];
        let (enc, uni) = compress(&data);
        assert_eq!(uni, vec![1,2,3]);
        assert_eq!(enc.len(), 4);
    }
}
