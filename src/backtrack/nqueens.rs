//! # Utility: Backtracking — N-Queens Solver
//!
//! Solve the classic n-Queens problem: place n queens so none attack.
//!
//! ## Algorithm Description
//!
//! - Recurse row by row.
//! - Track columns, and two diagonals to prune conflicts.
//!
//! ## Complexity
//!
//! - Time: O(n!) worst-case (heavily pruned)
//! - Space: O(n) recursion

/// Solve n-Queens: return all distinct solutions as vectors of column indices.
///
/// # Arguments
///
/// * `n` — Board size
///
/// # Returns
///
/// A vector of solutions, each a `Vec<usize>` of length `n`.
pub fn solve_nqueens(n: usize) -> Vec<Vec<usize>> {
    fn backtrack(
        row: usize,
        n: usize,
        cols: &mut Vec<bool>,
        d1: &mut Vec<bool>,
        d2: &mut Vec<bool>,
        sol: &mut Vec<usize>,
        out: &mut Vec<Vec<usize>>,
    ) {
        if row == n {
            out.push(sol.clone());
            return;
        }
        for c in 0..n {
            if !cols[c] && !d1[row + c] && !d2[row + n - c - 1] {
                cols[c] = true;
                d1[row + c] = true;
                d2[row + n - c - 1] = true;
                sol.push(c);

                backtrack(row + 1, n, cols, d1, d2, sol, out);

                sol.pop();
                cols[c] = false;
                d1[row + c] = false;
                d2[row + n - c - 1] = false;
            }
        }
    }

    let mut res = Vec::new();
    backtrack(
        0,
        n,
        &mut vec![false; n],
        &mut vec![false; 2*n],
        &mut vec![false; 2*n],
        &mut Vec::new(),
        &mut res,
    );
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nqueens_small() {
        let sol4 = solve_nqueens(4);
        assert_eq!(sol4.len(), 2);
    }
}
