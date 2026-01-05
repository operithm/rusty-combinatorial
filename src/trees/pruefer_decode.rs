//! # Chapter 9: Trees — Prüfer Code Decoding
//!
//! Decode a Prüfer sequence into a labeled tree edge list.
//!
//! ## Algorithm Description
//!
//! - Compute degrees from the code.
//! - Use a min‐heap of leaves to rebuild edges.

use std::{collections::BinaryHeap, cmp::Reverse};

/// Decode a Prüfer sequence into edges of a tree with `n` vertices.
///
/// # Arguments
///
/// * `code` — Prüfer sequence of length `n–2`
///
/// # Returns
///
/// A `Vec` of `(u,v)` edges for the reconstructed tree.
pub fn pruefer_decode(code: &[usize]) -> Vec<(usize, usize)> {
    let n = code.len() + 2;
    let mut deg = vec![1; n];
    for &v in code {
        deg[v] += 1;
    }

    let mut leaves = BinaryHeap::new();
    for i in 0..n {
        if deg[i] == 1 {
            leaves.push(Reverse(i));
        }
    }

    let mut edges = Vec::with_capacity(n - 1);
    for &v in code {
        let Reverse(u) = leaves.pop().unwrap();
        edges.push((u, v));
        deg[u] -= 1;
        deg[v] -= 1;
        if deg[v] == 1 {
            leaves.push(Reverse(v));
        }
    }
    let Reverse(a) = leaves.pop().unwrap();
    let Reverse(b) = leaves.pop().unwrap();
    edges.push((a, b));
    edges
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pruefer_decode_simple() {
        let code = vec![1, 1];
        let edges = pruefer_decode(&code);
        assert_eq!(edges.len(), 3);
    }
}
