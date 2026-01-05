//! LeetCode 17: Letter Combinations of a Phone Number
//! Generate all possible letter combinations from digit input

fn letter_combinations(digits: String) -> Vec<String> {
    if digits.is_empty() {
        return vec![];
    }
    let map = [
        "",     // 0
        "",     // 1
        "abc",  // 2
        "def",  // 3
        "ghi",  // 4
        "jkl",  // 5
        "mno",  // 6
        "pqrs", // 7
        "tuv",  // 8
        "wxyz", // 9
    ];

    let mut result = vec![];
    let mut current = String::new();

    fn backtrack(
        index: usize,
        digits: &str,
        map: &[&str],
        current: &mut String,
        result: &mut Vec<String>,
    ) {
        if index == digits.len() {
            result.push(current.clone());
            return;
        }
        let digit = digits.as_bytes()[index] - b'0';
        for c in map[digit as usize].chars() {
            current.push(c);
            backtrack(index + 1, digits, map, current, result);
            current.pop();
        }
    }

    backtrack(0, &digits, &map, &mut current, &mut result);
    result
}

fn main() {
    let digits = "23".to_string();
    let combos = letter_combinations(digits);
    for c in combos {
        println!("{}", c);
    }
}
