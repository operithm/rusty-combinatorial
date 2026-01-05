//! LeetCode 1655 - Distribute Repeating Integers
//! Count ways to distribute multiset of integers into k boxes such that each box non-empty.
//! This example implements a backtracking solution that works for small inputs.

use std::collections::HashMap;

fn ways_to_distribute(nums: Vec<i32>, k: i32) -> i64 {
    // We'll treat boxes as distinguishable for counting permutations of assignments.
    // Approach: backtrack over unique values, distribute counts across k boxes combinatorially.
    let mut freq = HashMap::new();
    for x in nums { *freq.entry(x).or_insert(0usize) += 1; }
    let vals: Vec<usize> = freq.values().cloned().collect();
    let mut ways = 0i64;

    // naive backtracking: for each occurrence assign to any box; prune symmetry minimally
    let n: usize = vals.iter().sum();
    fn helper(idx: usize, n: usize, k: usize, boxes: &mut Vec<usize>, vals_expanded: &Vec<usize>, ways: &mut i64) {
        if idx == n {
            if boxes.iter().all(|&x| x > 0) {
                *ways += 1;
            }
            return;
        }
        for b in 0..k {
            boxes[b] += 1;
            helper(idx + 1, n, k, boxes, vals_expanded, ways);
            boxes[b] -= 1;
        }
    }

    // expand multiset into individual items (works for small sizes)
    let mut expanded = Vec::new();
    for (&val, &cnt) in freq.iter() {
        for _ in 0..cnt { expanded.push(val as usize); }
    }
    let k_us = k as usize;
    let mut boxes = vec![0usize; k_us];
    helper(0, expanded.len(), k_us, &mut boxes, &expanded, &mut ways);
    ways
}

fn main() {
    let nums = vec![1,1,2];
    let k = 2;
    let cnt = ways_to_distribute(nums, k);
    println!("Ways to distribute = {}", cnt);
}
