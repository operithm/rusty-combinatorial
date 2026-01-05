//! # Utility: Gray Codes — Loopless Generation
//!
//! Generate Gray codes in numeric form without recursion, via bitwise formula.
//!
//! ## Algorithm
//!
//! - For i in 0..2ⁿ, Gray(i) = i XOR (i >> 1)
//!
//! ## Complexity
//!
//! - Time & Space: O(2ⁿ)

/// Produce Gray codes of length `n` as integer values (0..2ⁿ-1).
///
/// # Arguments
///
/// * `n` — Number of bits
///
/// # Returns
///
/// A vector of `u32` where each entry is the Gray‐coded integer.
///
/// # Example
///
/// ```
/// use rusty_combinatorial::gray::loopless::gray_loopless;
/// let g = gray_loopless(3);
/// assert_eq!(g[0], 0);
/// assert_eq!(g[1], 1);
/// assert_eq!(g[2], 3);
/// ```
pub fn gray_loopless(n: usize) -> Vec<u32> {
    let m = 1 << n;
    (0..m).map(|i| (i ^ (i >> 1)) as u32).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loopless_gray() {
        let g3 = gray_loopless(3);
        assert_eq!(g3.len(), 8);
        assert_eq!(g3[2], 0b011);
    }
}
