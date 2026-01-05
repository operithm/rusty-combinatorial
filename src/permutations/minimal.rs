//! # Chapter 1: Permutations — Minimal‐Change Order (Steinhaus–Johnson–Trotter)
//!
//! Generates permutations by swapping adjacent elements only, so each permutation
//! differs from the previous by a single adjacent transposition.
//!
//! ## Algorithm Description
//!
//! - Maintain an array of elements, each with a direction (← or →).
//! - At each step, find the largest “mobile” element (one that can move in its direction
//!   to a smaller neighbor).
//! - Swap it with that neighbor.
//! - Reverse the direction of all elements larger than the one moved.
//!
//! ## Complexity
//!
//! - Time: O(n!) total, O(1) amortized per permutation
//! - Space: O(n)

/// Generate all permutations of `n` elements in minimal-change (Gray‐code) order.
///
/// # Arguments
///
/// * `n` — Number of elements (producing permutations of `0..n`)
///
/// # Returns
///
/// A `Vec` containing each permutation as a `Vec<usize>` in sequence.
///
/// # Example
///
/// ```
/// use rusty_combinatorial::permutations::minimal::sjt_permutations;
/// let perms = sjt_permutations(3);
/// assert_eq!(perms.len(), 6);
/// assert_eq!(perms[0], vec![0, 1, 2]);
/// ```

pub fn sjt_permutations(n: usize) -> Vec<Vec<usize>> {
    #[derive(Clone)]
    struct Elem {
        val: usize,
        dir: i32, // -1 = left, +1 = right
    }

    if n == 0 {
        return vec![vec![]];
    }

    let mut a: Vec<Elem> = (0..n)
        .map(|v| Elem { val: v, dir: -1 })
        .collect();

    let mut out = Vec::with_capacity((1..=n).product());
    out.push(a.iter().map(|e| e.val).collect());

    loop {
        // find the largest mobile element
        let mut mobile_idx: Option<usize> = None;
        let mut mobile_val = None;

        for i in 0..n {
            let e = &a[i];
            let j = match e.dir {
                -1 => {
                    if i == 0 { continue; }
                    i - 1
                }
                1 => {
                    if i == n - 1 { continue; }
                    i + 1
                }
                _ => unreachable!(),
            };
            if a[j].val < e.val {
                if mobile_val.map_or(true, |mv| e.val > mv) {
                    mobile_val = Some(e.val);
                    mobile_idx = Some(i);
                }
            }
        }

        let i = match mobile_idx {
            Some(idx) => idx,
            None => break,
        };

        // swap mobile element
        let j = (i as i32 + a[i].dir) as usize;
        a.swap(i, j);

        // reverse directions of larger values
        let moved_val = a[j].val;
        for e in a.iter_mut() {
            if e.val > moved_val {
                e.dir = -e.dir;
            }
        }

        out.push(a.iter().map(|e| e.val).collect());
    }

    out
}
