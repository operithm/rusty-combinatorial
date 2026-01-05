//! LeetCode 47: Permutations II
//! Generate all unique permutations of an array that may contain duplicates

fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result = vec![];
    nums.sort();
    let mut used = vec![false; nums.len()];
    let mut current = vec![];

    fn backtrack(
        nums: &[i32],
        used: &mut [bool],
        current: &mut Vec<i32>,
        result: &mut Vec<Vec<i32>>,
    ) {
        if current.len() == nums.len() {
            result.push(current.clone());
            return;
        }
        for i in 0..nums.len() {
            if used[i] || (i > 0 && nums[i] == nums[i - 1] && !used[i - 1]) {
                continue;
            }
            used[i] = true;
            current.push(nums[i]);
            backtrack(nums, used, current, result);
            current.pop();
            used[i] = false;
        }
    }

    backtrack(&nums, &mut used, &mut current, &mut result);
    result
}

fn main() {
    let nums = vec![1, 1, 2];
    let perms = permute_unique(nums);
    for p in perms {
        println!("{:?}", p);
    }
}
