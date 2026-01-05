
//! LeetCode 2266 - Count Number of Texts
//!
//! Dynamic programming on runs of identical digits.
//!

const MOD: i64 = 1_000_000_007;

pub fn count_texts(pressed_keys: &str) -> i32 {
    let n = pressed_keys.len();

    let mut dp3 = vec![0i64; n + 1];
    let mut dp4 = vec![0i64; n + 1];
    dp3[0] = 1;
    dp4[0] = 1;

    for i in 1..=n {
        for j in 1..=3 {
            if i >= j {
                dp3[i] = (dp3[i] + dp3[i - j]) % MOD;
            }
        }
        for j in 1..=4 {
            if i >= j {
                dp4[i] = (dp4[i] + dp4[i - j]) % MOD;
            }
        }
    }

    let bytes = pressed_keys.as_bytes();
    let mut ans = 1i64;
    let mut i = 0;

    while i < n {
        let mut j = i;
        while j < n && bytes[j] == bytes[i] {
            j += 1;
        }
        let len = j - i;
        let digit = bytes[i] as char;

        let ways = if digit == '7' || digit == '9' {
            dp4[len]
        } else {
            dp3[len]
        };

        ans = ans * ways % MOD;
        i = j;
    }

    ans as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_examples() {
        assert_eq!(count_texts("22233"), 8);
        assert_eq!(count_texts("7777"), 8);
        assert_eq!(count_texts("9999"), 8);
    }

    #[test]
    fn test_single_digit() {
        assert_eq!(count_texts("2"), 1);
        assert_eq!(count_texts("7"), 1);
    }

    #[test]
    fn test_large_run() {
        let s = "2".repeat(1000);
        let result = count_texts(&s);
        assert!(result > 0); // sanity check
    }
}


fn main() {
    let cases = [
        "22233",
        "222222222222222222222222222222222222",
        "7777",
        "9999",
        "88888888888888888888888888888999999999999999999999999999994444444444444444444444444444488888888888888888888888888888555555555555555555555555555556666666666666666666666666666666666666666666666666666666666222222222222222222222222222226666666666666666666666666666699999999999999999999999999999888888888888888888888888888885555555555555555555555555555577777777777777777777777777777444444444444444444444444444444444444444444444444444444444433333333333333333333333333333555555555555555555555555555556666666666666666666666666666644444444444444444444444444444999999999999999999999999999996666666666666666666666666666655555555555555555555555555555444444444444444444444444444448888888888888888888888888888855555555555555555555555555555555555555555555555555555555555555555555555555555555555999999999999999555555555555555555555555555554444444444444444444444444444444555",
        "444479999555588866",
    ];

    for s in cases {
        println!("pressedKeys = {s}, ways = {}", count_texts(s));
    }
}


