//! # Utility: Backtracking — Subset Sum
//!
//! Find all subsets of `nums` summing to `target`.
//!
//! ## Algorithm Description
//!
//! - Recurse with include/exclude choices.
//! - Prune when sum exceeds target.
//!
//! ## Complexity
//!
//! - Time: O(2ⁿ) worst-case, pruned to fewer branches
//! - Space: O(n) recursion

/// Return all subsets of `nums` that sum to `target`.
///
/// # Arguments
///
/// * `nums` — Slice of integers
/// * `target` — Sum to achieve
///
/// # Returns
///
/// A vector of subsets (Vec<Vec<i32>>) meeting the target.
///
/// # Example
///
/// ```
/// use rusty_combinatorial::backtrack::subset_sum::subset_sum;
/// let sol = subset_sum(&[2,3,6,7], 7);
/// assert!(sol.contains(&vec![7]));
/// ```
pub fn subset_sum(nums: &[i32], target: i32) -> Vec<Vec<i32>> {
    fn helper(
        idx: usize,
        nums: &[i32],
        curr: &mut Vec<i32>,
        sum: i32,
        target: i32,
        out: &mut Vec<Vec<i32>>,
    ) {
        if sum == target {
            out.push(curr.clone());
            return;
        }
        if idx == nums.len() || sum > target {
            return;
        }
        // Exclude
        helper(idx + 1, nums, curr, sum, target, out);
        // Include
        curr.push(nums[idx]);
        helper(idx + 1, nums, curr, sum + nums[idx], target, out);
        curr.pop();
    }

    let mut res = Vec::new();
    helper(0, nums, &mut Vec::new(), 0, target, &mut res);
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subset_sum() {
        let result = subset_sum(&[1,2,3], 3);
        assert!(result.contains(&vec![1,2]) || result.contains(&vec![3]));
    }
}
