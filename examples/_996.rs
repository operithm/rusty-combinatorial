//! LeetCode 996: Number of Squareful Arrays
//! Count all permutations where adjacent sums are perfect squares
use::rusty_combinatorial::permutations::lex;
use rusty_combinatorial::permutations::lex::next_lex_perm;

fn is_square(n: usize) -> bool {
    let r = (n as f64).sqrt() as usize;
    r * r == n
}

fn is_squareful(a: &[usize]) -> bool {
    a.windows(2).all(|w| is_square(w[0] + w[1]))
}

fn num_squareful_perms(nums: Vec<usize>) -> i32 {
    let mut nums = nums;
    nums.sort();
    let mut count = 0;
    let mut seen = std::collections::HashSet::new();
    loop {
        if is_squareful(&nums) && seen.insert(nums.clone()) {
            count += 1;
        }
        if !next_lex_perm(&mut nums) {
            break;
        }
    }
    count
}

fn main() {
    let nums = vec![1, 17, 8];
    let count = num_squareful_perms(nums);
    println!("Number of squareful arrays: {}", count);
}
