//! # Chapter 1: Permutations — Ranking & Unranking (Lehmer Code)
//!
//! Assign a unique integer rank to each permutation in lex order, and
//! reconstruct a permutation from its rank.
//!
//! ## Algorithm Description
//!
//! **Ranking**: For each position, count how many unused smaller values remain,
//!           multiply by factorials (mixed‐radix).
//! **Unranking**: Decompose the rank into mixed‐radix digits and pick elements
//!             from a list of available values.
//!
//! ## Complexity
//!
//! - Time: O(n²)
//! - Space: O(n)

/// Compute the lexicographic rank of permutation `p` (0‐based).
///
/// # Arguments
///
/// * `p` — A slice of a permutation of `0..n`
///
/// # Returns
///
/// Rank in `0..n!`
///
/// # Example
///
/// ```
/// use rusty_combinatorial::permutations::ranking::rank_perm;
/// let r = rank_perm(&[2,0,1]);
/// assert_eq!(r, 4);
/// ```
/// Compute rank of permutation p (0-based), lexicographic order.
pub fn rank_perm(p: &[usize]) -> usize {
    let n = p.len();
    let f = factorials(n);
    let mut rank = 0usize;
    let mut used = vec![false; n];
    for i in 0..n {
        let mut cnt = 0usize;
        for j in 0..p[i] {
            if !used[j] {
                cnt += 1;
            }
        }
        rank += cnt * f[n - 1 - i];
        used[p[i]] = true;
    }
    rank
}

/// Reconstruct the permutation of length `n` with given `rank` (0‐based).
///
/// # Arguments
///
/// * `rank` — Rank in `0..n!`
/// * `n` — Permutation length
///
/// # Returns
///
/// The corresponding permutation as `Vec<usize>`.
///
/// # Example
///
/// ```
/// use rusty_combinatorial::permutations::ranking::unrank_perm;
/// let p = unrank_perm(4, 3);
/// assert_eq!(p, vec![2,0,1]);
/// ```
/// Reconstruct permutation of length n with given rank (0-based)
pub fn unrank_perm(mut rank: usize, n: usize) -> Vec<usize> {
    let f = factorials(n);
    let mut avail: Vec<usize> = (0..n).collect();
    let mut out = Vec::with_capacity(n);
    for i in 0..n {
        let block = f[n - 1 - i];
        let idx = rank / block;
        rank %= block;
        out.push(avail.remove(idx));
    }
    out
}

/// compute factorials up to n
fn factorials(n: usize) -> Vec<usize> {
    let mut f = vec![1usize; n + 1];
    for i in 1..=n {
        f[i] = f[i - 1] * i;
    }
    f
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank_unrank_roundtrip() {
        let p = vec![2, 0, 1];
        let r = rank_perm(&p);
        let q = unrank_perm(r, 3);
        assert_eq!(p, q);
    }
}
