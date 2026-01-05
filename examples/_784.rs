//! Leetcode 784 (Gray Code variant): Letter Case Permutation
//! Generate all strings with letter case changes (binary choice tree)

fn letter_case_permutation(s: String) -> Vec<String> {
    let mut result = vec![];
    let chars: Vec<char> = s.chars().collect();

    fn backtrack(i: usize, current: &mut Vec<char>, chars: &[char], result: &mut Vec<String>) {
        if i == chars.len() {
            result.push(current.iter().collect());
            return;
        }
        if chars[i].is_alphabetic() {
            current.push(chars[i].to_ascii_lowercase());
            backtrack(i + 1, current, chars, result);
            current.pop();
            current.push(chars[i].to_ascii_uppercase());
            backtrack(i + 1, current, chars, result);
            current.pop();
        } else {
            current.push(chars[i]);
            backtrack(i + 1, current, chars, result);
            current.pop();
        }
    }

    backtrack(0, &mut vec![], &chars, &mut result);
    result
}

fn main() {
    let result = letter_case_permutation("a1b2".to_string());
    println!("Generated permutations:");
    for s in result {
        println!("{}", s);
    }
}
