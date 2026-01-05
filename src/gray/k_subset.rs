//! # Utility: Gray Codes — k-Subset Gray Code (Chase’s Algorithm)
//!
//! Generate all k-subsets of {0,1,…,n-1} such that each successive subset differs
//! from the previous one in exactly **one exchanged element** (minimal-change order).
//!
//! ## References
//! - Chase, “Algorithm 382: Combinations of M out of N objects.” CACM 1972.
//! - Knuth TAOCP Vol.4A, 7.2.1.3 (Algorithm L, F, and C for combinatorial Gray codes).

/// Generate Gray-code order of all k-subsets of an n-element set.
///
/// # Arguments
///
/// * `n` – Total elements
/// * `k` – Subset size
///
/// # Returns
///
/// A vector of subsets in minimal-change Gray-code order.
///
/// # Example
///
/// ```
/// use rusty_combinatorial::gray::k_subset::k_subset_gray;
/// let subsets = k_subset_gray(4, 2);
/// assert_eq!(subsets.len(), 6);
/// assert_eq!(subsets[0], vec![0,1]);
/// ```

pub fn k_subset_gray(n: usize, k: usize) -> Vec<Vec<usize>> {
    // Base cases
    if k > n {
        return vec![];
    }
    if k == 0 {
        return vec![vec![]];
    }
    if k == n {
        return vec![(0..n).collect()];
    }

    // Recursive construction
    let a = k_subset_gray(n - 1, k); // subsets not containing n-1
    let mut b = k_subset_gray(n - 1, k - 1); // subsets of size k-1 in {0..n-2}

    let mut out = Vec::with_capacity(a.len() + b.len());
    out.extend(a.into_iter());
    // append n-1 to reversed b
    while let Some(mut subset) = b.pop() {
        subset.push(n - 1);
        out.push(subset);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k_subset_gray() {
        let subsets = k_subset_gray(4, 2);
        // 6 subsets of 4 choose 2
        assert_eq!(subsets.len(), 6);

        // check minimal change: symmetric difference == 2
        for w in subsets.windows(2) {
            let a = &w[0];
            let b = &w[1];
            let mut diff = 0usize;
            for i in 0..4 {
                let in_a = a.contains(&i);
                let in_b = b.contains(&i);
                if in_a != in_b {
                    diff += 1;
                }
            }
            assert_eq!(diff, 2);
        }

        // basic correctness: ensure all combinations are present
        use std::collections::HashSet;
        let set: HashSet<Vec<usize>> = subsets.into_iter().map(|mut s| { s.sort_unstable(); s }).collect();
        assert_eq!(set.len(), 6);
    }
}
