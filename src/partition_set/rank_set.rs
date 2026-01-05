//! # Chapter 7: Set Partitions — Counting (Stirling Numbers)
//!
//! Compute and use Stirling numbers of the second kind to count and rank set partitions.
//!
//! ## Algorithm Description
//!
//! - Use recurrence: S(n,k) = k·S(n-1,k) + S(n-1,k-1)
//!
//! ## Complexity
//!
//! - Time: O(n·k)
//! - Space: O(n·k)

/// Compute the Stirling number of the second kind S(n, k).
///
/// # Arguments
///
/// * `n` — Number of elements
/// * `k` — Number of subsets
///
/// # Returns
///
/// The number of ways to partition `n` labeled items into `k` nonempty subsets.
///
/// # Example
///
/// ```
/// use rusty_combinatorial::partition_set::rank_set::stirling;
/// assert_eq!(stirling(3,2), 3);
/// ```
pub fn stirling(n: usize, k: usize) -> usize {
    let mut dp = vec![vec![0; k + 1]; n + 1];
    dp[0][0] = 1;
    for i in 1..=n {
        for j in 1..=k.min(i) {
            dp[i][j] = j * dp[i - 1][j] + dp[i - 1][j - 1];
        }
    }
    dp[n][k]
}

/// Alias that counts S(n, k).
pub fn count_set_partitions(n: usize, k: usize) -> usize {
    stirling(n, k)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stirling_small() {
        assert_eq!(count_set_partitions(3,2), 3);
        assert_eq!(count_set_partitions(4,2), 7);
    }
}
