LeetCode **2266. Count Number of Texts** is a very nice example of **dynamic programming on runs**, and it fits perfectly with the combinatorial thinking you’ve been building.

I’ll explain it **from first principles**, connect it to known algorithms, and then give a **clean Rust-style solution outline**.

---

## Problem recap (in plain terms)

You are given a string `pressedKeys` consisting of digits `2`–`9`.

Each digit corresponds to letters on a phone keypad:

| Digit | Letters     | Max presses |
| ----: | ----------- | ----------- |
| 2–6,8 | abc…        | 3           |
|   7,9 | pqrs / wxyz | 4           |

You can group **consecutive identical digits** into chunks of size `1` to `k`
(where `k = 3` or `4` depending on the digit).

👉 Count how many different text messages could produce exactly `pressedKeys`.

Return the count modulo `1_000_000_007`.

---

## Key insight (the algorithmic core)

### 🔑 Independence by runs

The string splits naturally into **maximal runs** of identical digits:

```
"22233" → ["222", "33"]
```

Crucially:

* Each run can be decoded **independently**
* The total number of texts is the **product** of possibilities for each run

This immediately suggests:

> **Dynamic programming on the length of each run**

---

## Subproblem: number of ways to decode one run

Let:

* `len` = length of the run
* `k` = max group size (3 or 4)

Define:

```
dp[i] = number of ways to split i identical key presses
```

### Recurrence

For `i ≥ 1`:

```
dp[i] = dp[i-1] + dp[i-2] + ... + dp[i-k]
```

(with bounds checking)

Base case:

```
dp[0] = 1
```

This is a **bounded composition counting problem**, very similar to:

* counting tilings
* counting restricted partitions
* Fibonacci-like recurrences

---

## Example

For digit `2` (`k = 3`) and run length `4`:

```
dp[0] = 1
dp[1] = 1          (1)
dp[2] = 2          (1+1, 2)
dp[3] = 4          (1+1+1, 1+2, 2+1, 3)
dp[4] = 7          (sum of dp[1..3])
```

So `"2222"` has **7** interpretations.

---

## Full algorithm

1. Precompute DP arrays:

    * one for `k = 3`
    * one for `k = 4`
2. Scan `pressedKeys` and extract runs
3. For each run:

    * multiply the result by `dp[run_length]`
4. Apply modulo arithmetic

---

## Time & space complexity

* **Time:** `O(n)`
* **Space:** `O(n)` (or `O(1)` rolling DP)
* Optimal and required for large inputs

---

## Rust-style solution (clear & idiomatic)

```rust
const MOD: i64 = 1_000_000_007;

pub fn count_texts(pressed_keys: String) -> i32 {
    let n = pressed_keys.len();

    // dp3[i]: ways for length i with max group size 3
    // dp4[i]: ways for length i with max group size 4
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
```

---

## How this connects to your crate

This problem directly uses ideas from:

* **Compositions with bounded parts** (Chapter on partitions / combinations)
* **Run-length factorization**
* **DP with small transition width**
* **Gray / phone-keypad combinatorics**

It’s a perfect candidate for your **LeetCode chapter** as a:

> “DP interpretation of combinatorial encoding”

---

## Final mental model

> “Count the ways to break each run into valid chunks, then multiply.”

Once you see it that way, the problem becomes straightforward and elegant.

If you want next, I can:

* rewrite this using **rolling DP**
* express it in **CLRS-style pseudocode**
* connect it to **restricted integer compositions**
* add it as `examples/leetcode_2266.rs` using your crate style

Just tell me 👍
