//! # Utility: Gray Codes — Binary Reflected Gray Code
//!
//! Generate the classic binary reflected Gray codes of length `n`.
//!
//! ## Algorithm
//!
//! - Recursively prepend `0` to the codes of size `n-1`
//! - Then prepend `1` to the reversed codes of size `n-1`
//!
//! This ensures only one bit changes between successive codes.
//!
//! ## Complexity
//!
//! - Time & Space: O(2ⁿ · n)

/// Produce the binary reflected Gray code sequence for `n` bits.
///
/// # Arguments
///
/// * `n` — Number of bits per codeword
///
/// # Returns
///
/// A vector of bit‐vectors (Vec<u8>) of length 2ⁿ.
///
/// # Example
///
/// ```
/// use rusty_combinatorial::gray::binary::binary_reflected_gray;
/// let codes = binary_reflected_gray(3);
/// assert_eq!(codes.len(), 8);
/// assert_eq!(codes[0], vec![0,0,0]);
/// assert_eq!(codes[1], vec![0,0,1]);
/// ```
pub fn binary_reflected_gray(n: usize) -> Vec<Vec<u8>> {
    if n == 0 {
        return vec![vec![]];
    }
    let prev = binary_reflected_gray(n - 1);
    let mut out = Vec::with_capacity(1 << n);
    for code in &prev {
        let mut with_zero = vec![0];
        with_zero.extend(code);
        out.push(with_zero);
    }
    for code in prev.iter().rev() {
        let mut with_one = vec![1];
        with_one.extend(code);
        out.push(with_one);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_gray_small() {
        let g2 = binary_reflected_gray(2);
        assert_eq!(g2, vec![vec![0,0], vec![0,1], vec![1,1], vec![1,0]]);
    }
}
