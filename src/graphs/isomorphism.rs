//! # Chapter 8: Unlabeled Graphs — Isomorphism Checking
//!
//! Check whether two graphs are isomorphic by brute‐force vertex permutations.
//!
//! ## Algorithm Description
//!
//! - Two `n×n` adjacency matrices represent isomorphic graphs if there exists
//!   a permutation `π` of `{0..n}` such that `G1[i][j] == G2[π(i)][π(j)]`.
//! - Try all `n!` permutations (factorial time).
//!
//! ## Complexity
//!
//! - Time: O(n! · n²)

use crate::permutations::lex::next_lex_perm;

/// Test whether two adjacency matrices represent isomorphic graphs.
///
/// # Arguments
///
/// * `g1`, `g2` — Square adjacency matrices of the same size
///
/// # Returns
///
/// `true` if graphs are isomorphic, `false` otherwise.
///
/// # Example
///
/// ```
/// use rusty_combinatorial::graphs::isomorphism::are_isomorphic;
/// let g1 = vec![vec![false,true],[true,false].to_vec()];
/// let g2 = vec![vec![false,true],[true,false].to_vec()];
/// assert!(are_isomorphic(&g1, &g2));
/// ```
pub fn are_isomorphic(g1: &[Vec<bool>], g2: &[Vec<bool>]) -> bool {
    let n = g1.len();
    if n != g2.len() { return false; }
    let mut perm: Vec<usize> = (0..n).collect();
    loop {
        let mut match_all = true;
        for i in 0..n {
            for j in 0..n {
                if g1[i][j] != g2[perm[i]][perm[j]] {
                    match_all = false;
                    break;
                }
            }
            if !match_all { break; }
        }
        if match_all { return true; }
        if !next_lex_perm(&mut perm) {
            break;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_isomorphic() {
        let g1 = vec![
            vec![false,true,false],
            vec![true,false,true],
            vec![false,true,false],
        ];
        let g2 = vec![
            vec![false,false,true],
            vec![false,false,true],
            vec![true,true,false],
        ];
        assert!(are_isomorphic(&g1, &g2));
    }
}
