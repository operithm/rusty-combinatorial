//! LeetCode 90: Subsets II
//! Generate all unique subsets from a list that may contain duplicates

fn subsets_with_dup(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort();
    let mut result = vec![];
    let mut current = vec![];

    fn backtrack(
        start: usize,
        nums: &[i32],
        current: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        result.push(current.clone());
        for i in start..nums.len() {
            if i > start && nums[i] == nums[i - 1] {
                continue;
            }
            current.push(nums[i]);
            backtrack(i + 1, nums, current, result);
            current.pop();
        }
    }

    backtrack(0, &nums, &mut current, &mut result);
    result
}

fn main() {
    let nums = vec![1, 2, 2];
    let subsets = subsets_with_dup(nums);
    for s in subsets {
        println!("{:?}", s);
    }
}
