//! # Chapter 5: Combinations — Ranking & Unranking
//!
//! Use the combinatorial number system to map each k-combination to
//! a unique rank in [0, C(n, k)] and back.
//!
//! ## Algorithm Description
//!
//! **Ranking**: For each chosen index, count how many skipped elements
//!   contribute binomially to the rank.
//! **Unranking**: Greedily subtract binomial coefficients to reconstruct
//!   each index in order.
//!
//! ## Complexity
//!
//! - Time: O(k²) (or O(k log n) with fast binomial)
//! - Space: O(1) extra

/// Compute C(n, k) = n choose k in O(k) time.
///
/// # Example
///
/// ```
/// use::rusty_combinatorial::combinations::ranking::binom;
/// assert_eq!(binom(5, 3), 10);
/// ```
pub fn binom(n: usize, k: usize) -> usize {
    if k > n { return 0; }
    let mut res = 1;
    for i in 0..k {
        res = res * (n - i) / (i + 1);
    }
    res
}

/// Rank the k-combination `comb` among all combinations of `n`.
///
/// # Arguments
///
/// * `comb` — Sorted slice of k indices
/// * `n` — Total number of elements
///
/// # Returns
///
/// The 0-based lex rank.
///
/// # Example
///
/// ```
/// use::rusty_combinatorial::combinations::ranking::rank_comb;
/// let rank = rank_comb(&[1,3,4], 6);
/// assert_eq!(rank, 13);
/// ```
pub fn rank_comb(comb: &[usize], n: usize) -> usize {
    let k = comb.len();
    let mut rank = 0;
    let mut prev = 0;
    for (i, &c) in comb.iter().enumerate() {
        for j in prev..c {
            rank += binom(n - j - 1, k - i - 1);
        }
        prev = c + 1;
    }
    rank
}

/// Unrank to retrieve the k-combination at position `rank` among C(n,k).
///
/// # Arguments
///
/// * `rank` — 0-based rank
/// * `n` — Total number of elements
/// * `k` — Combination size
///
/// # Returns
///
/// The combination as a sorted vector of indices.
///
/// # Example
///
/// ```
/// use rusty_combinatorial::combinations::ranking::unrank_comb;
/// let comb = unrank_comb(13, 6, 3);
/// assert_eq!(comb, vec![1,3,4]);
/// ```
pub fn unrank_comb(mut rank: usize, n: usize, k: usize) -> Vec<usize> {
    let mut comb = Vec::with_capacity(k);
    let mut prev = 0;
    for i in 0..k {
        for j in prev..n {
            let c = binom(n - j - 1, k - i - 1);
            if rank < c {
                comb.push(j);
                prev = j + 1;
                break;
            }
            rank -= c;
        }
    }
    comb
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank_unrank() {
        let comb = vec![1, 3, 4];
        let r = rank_comb(&comb, 6);
        let u = unrank_comb(r, 6, comb.len());
        assert_eq!(comb, u);
    }
}
