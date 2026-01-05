//! # Chapter 10: Young Tableaux — Hook‐Length Formula
//!
//! Count standard Young tableaux of a shape via the hook‐length formula.
//!
//! ## Algorithm Description
//!
//! - Compute hook‐length for each cell in the Ferrers diagram.
//! - Result = n! / ∏ hooks.
//!
//! ## Complexity
//!
//! - Time: O(n²)

/// Compute the number of SYTs for the given shape using hook‐length.
///
/// # Arguments
///
/// * `shape` — 2D `Vec` of `bool` indicating diagram cells.
///
/// # Returns
///
/// Number of standard Young tableaux (usize).
pub fn hook_length(shape: &[Vec<bool>]) -> usize {
    let mut hooks = Vec::new();
    for i in 0..shape.len() {
        for j in 0..shape[i].len() {
            if shape[i][j] {
                let mut h = 1;
                // Down
                for ii in i + 1..shape.len() {
                    if j < shape[ii].len() && shape[ii][j] {
                        h += 1;
                    } else {
                        break;
                    }
                }
                // Right
                for jj in j + 1..shape[i].len() {
                    if shape[i][jj] {
                        h += 1;
                    } else {
                        break;
                    }
                }
                hooks.push(h);
            }
        }
    }
    let n = hooks.len();
    let num = (1..=n).product::<usize>();
    let den = hooks.into_iter().product::<usize>();
    num / den
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hook_length_simple() {
        let shape = vec![vec![true, true], vec![true]];
        assert_eq!(hook_length(&shape), 2);
    }
}
