//! # Chapter 8: Unlabeled Graphs — Generation
//!
//! Enumerate all simple undirected graphs on `n` vertices, represented as
//! adjacency matrices without self-loops or multi-edges.
//!
//! ## Algorithm Description
//!
//! - There are `n(n-1)/2` possible edges.
//! - Treat each graph as a bitmask of length `m = n(n-1)/2`.
//! - For each integer `0..2^m`, build the symmetric adjacency matrix.
//!
//! ## Complexity
//!
//! - Time: O(2^{n(n-1)/2} · n^2)
//! - Space: O(n^2)

/// Generate all simple undirected graphs on `n` vertices.
///
/// # Arguments
///
/// * `n` — Number of vertices
///
/// # Returns
///
/// A `Vec` of adjacency matrices (`Vec<Vec<bool>>`), one per graph.
///
/// # Example
///
/// ```
/// use rusty_combinatorial::graphs::generate::generate_graphs;
/// let graphs = generate_graphs(3);
/// assert_eq!(graphs.len(), 8);
/// ```
pub fn generate_graphs(n: usize) -> Vec<Vec<Vec<bool>>> {
    let m = n * (n - 1) / 2;
    let mut result = Vec::with_capacity(1 << m);
    for bits in 0..(1 << m) {
        let mut mat = vec![vec![false; n]; n];
        let mut idx = 0;
        for i in 0..n {
            for j in (i + 1)..n {
                if (bits >> idx) & 1 == 1 {
                    mat[i][j] = true;
                    mat[j][i] = true;
                }
                idx += 1;
            }
        }
        result.push(mat);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_graphs_count() {
        let gs = generate_graphs(3);
        assert_eq!(gs.len(), 8);
    }
}
