//! # Chapter 10: Young Tableaux — Generation
//!
//! Enumerate all standard Young tableaux (SYT) of a given Ferrers shape.
//!
//! ## Algorithm Description
//!
//! - Backtrack by placing numbers 1..n into the tableau grid.
//! - Ensure each row and column is strictly increasing.
//!
//! ## Complexity
//!
//! - Time: O(n! / ∏ hooks) (number of SYTs)
//! - Space: O(n) recursion + tableau storage

/// Generate all standard Young tableaux for the given row lengths.
///
/// # Arguments
///
/// * `shape` — Slice of row lengths, e.g. `[3, 2]`
///
/// # Returns
///
/// A `Vec` of tableaux, each a `Vec<Vec<usize>>`.

pub fn generate_syt(shape: &[usize]) -> Vec<Vec<Vec<usize>>> {
    let _n: usize = shape.iter().sum();
    let mut table: Vec<Vec<usize>> = shape.iter().map(|&l| vec![0; l]).collect();
    let mut result = Vec::new();

    fn next_empty(table: &[Vec<usize>]) -> Option<(usize, usize)> {
        for i in 0..table.len() {
            for j in 0..table[i].len() {
                if table[i][j] == 0 {
                    return Some((i, j));
                }
            }
        }
        None
    }

    fn valid(table: &[Vec<usize>], i: usize, j: usize, val: usize) -> bool {
        if j > 0 && table[i][j - 1] != 0 && table[i][j - 1] >= val {
            return false;
        }
        if i > 0 && j < table[i - 1].len() && table[i - 1][j] != 0 && table[i - 1][j] >= val {
            return false;
        }
        true
    }

    fn backtrack(
        val: usize,
        table: &mut Vec<Vec<usize>>,
        result: &mut Vec<Vec<Vec<usize>>>,
    ) {
        if let Some((i, j)) = next_empty(table) {
            for v in val..=table.len() * table[0].len() + 1 {
                if valid(table, i, j, v) {
                    table[i][j] = v;
                    backtrack(v + 1, table, result);
                    table[i][j] = 0;
                }
            }
        } else {
            result.push(table.clone());
        }
    }

    backtrack(1, &mut table, &mut result);
    result
}