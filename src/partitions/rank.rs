//! # Chapter 6: Integer Partitions — Ranking
//!
//! Assign a lexicographic rank to an integer partition of `n` using dynamic
//! programming table of partition counts.
//!
//! ## Algorithm Description
//!
//! 1. Build table `P[i][j]` = number of partitions of `i` with maximum part `j`.
//! 2. To rank partition `[p₁, p₂, …]`, for each part `pₖ`, count how many partitions
//!    start with smaller first part, then recurse on remainder.
//!
//! ## Complexity
//!
//! - Time: O(n²)
//! - Space: O(n²)
//!
//! Rank integer partitions in lexicographic order (first parts compared first).
//! Uses partition table P[i][j] = number of partitions of i with parts ≤ j.
//!
/// Build the partition count table P(i,j) for 0..=n
///
/// `P[i][j]` = count of partitions of `i` using parts ≤ `j`.

fn partition_table(n: usize) -> Vec<Vec<usize>> {
    let mut p = vec![vec![0usize; n + 1]; n + 1];
    p[0].iter_mut().for_each(|x| *x = 1);
    for i in 1..=n {
        for j in 1..=n {
            p[i][j] = p[i][j - 1];
            if j <= i {
                p[i][j] = p[i][j].saturating_add(p[i - j][j]);
            }
        }
    }
    p
}


/// Compute the lexicographic rank of a partition of `n`.
///
/// # Arguments
///
/// * `part` — A slice of positive integers in non‐increasing order summing to `n`
/// * `n` — The integer being partitioned
///
/// # Returns
///
/// The 0‐based lex rank among all partitions of `n`.
///
/// # Example
///
/// ```
/// use rusty_combinatorial::partitions::rank::rank_partition;
/// let rank = rank_partition(&[2, 2], 4);
/// assert!(rank < 5); // there are 5 partitions of 4
/// ```

pub fn rank_partition(part: &[usize], n: usize) -> usize {
    let p = partition_table(n);
    let mut rank = 0usize;
    let mut rem = n;
    let mut prev_max = n;

    for &x in part {
        // valid values for this part are in 1..=prev_max and must sum to rem
        // We count partitions whose current part v is > x (i.e., v in (x+1)..=prev_max).
        if x + 1 <= prev_max {
            for v in (x + 1)..=prev_max {
                if v <= rem {
                    rank = rank.saturating_add(p[rem - v][v]);
                }
            }
        }
        rem = rem - x;
        prev_max = x;
    }
    rank
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank_partition_simple() {
        let r0 = rank_partition(&[4], 4);
        let r1 = rank_partition(&[3, 1], 4);
        let r2 = rank_partition(&[2, 2], 4);
        assert!(r0 < r1 && r1 < r2);
    }
}

