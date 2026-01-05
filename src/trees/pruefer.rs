//! # Chapter 9: Trees — Prüfer Code Encoding
//!
//! Convert a labeled tree into its Prüfer sequence (bijective encoding).
//!
//! ## Algorithm Description
//!
//! - Compute degrees and adjacency.
//! - Repeatedly remove the smallest leaf and record its neighbor.
//!
//! ## Complexity
//!
//! - Time: O(n log n) using a min‐heap or sorted set
//! - Space: O(n)

/// Encode a tree on `n` vertices (0..n-1) given its edge list.
///
/// # Arguments
///
/// * `n` — Number of vertices
/// * `edges` — Slice of `(u, v)` pairs
///
/// # Returns
///
/// Prüfer code as `Vec<usize>` of length `n-2`.
pub fn pruefer_encode(n: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    use std::{collections::BinaryHeap, cmp::Reverse};
    let mut deg = vec![0; n];
    let mut adj = vec![vec![]; n];
    for &(u, v) in edges {
        deg[u] += 1;
        deg[v] += 1;
        adj[u].push(v);
        adj[v].push(u);
    }

    let mut leaves = BinaryHeap::new();
    for i in 0..n {
        if deg[i] == 1 {
            leaves.push(Reverse(i));
        }
    }

    let mut code = Vec::with_capacity(n - 2);
    for _ in 0..n - 2 {
        let Reverse(u) = leaves.pop().unwrap();
        for &v in &adj[u] {
            if deg[v] > 0 {
                code.push(v);
                deg[v] -= 1;
                if deg[v] == 1 {
                    leaves.push(Reverse(v));
                }
                break;
            }
        }
        deg[u] = 0;
    }
    code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pruefer_encode_simple() {
        let edges = vec![(0, 1), (1, 2), (1, 3)];
        let code = pruefer_encode(4, &edges);
        assert_eq!(code, vec![1, 1]);
    }
}
