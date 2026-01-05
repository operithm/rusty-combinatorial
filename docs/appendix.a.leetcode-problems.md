Here's a curated selection of **LeetCode Hard** problems that can be solved using the algorithms and techniques implemented in our crate. We'll organize them into a new chapter: **LeetCode Problems**, mapping each problem to the relevant combinatorial method.

---

## 🧩 Chapter: LeetCode Problems

### 1. **Backtracking**

* **37. Sudoku Solver (Hard)**
  A classic constraint-solving task using backtracking ([books.halfrost.com][1], [github.com][2]). Our `backtrack::subset_sum` and `backtrack::nqueens` modules showcase similar recursive pruning approaches.

* **51. N‑Queens** & **52. N‑Queens II** (both Hard)
  Exactly matches our `backtrack::nqueens` implementation ([books.halfrost.com][1]). You can directly adapt its solver for LeetCode versions.

* **212. Word Search II (Hard)**
  Complex backtracking on grids with dictionary lookup, akin to subset/graph traversal patterns ([books.halfrost.com][1]).

---

### 2. **Permutations & Combinations**

* **90. Subsets II** (Medium) & **47. Permutations II** (Medium) & **17. Letter Combinations of a Phone Number** (Medium)
  These are classic uses of permutations/combinations and constraint-driven enumeration ([books.halfrost.com][1]). Our modules (`gray`, `combinations`, `ranking`, `minimal`) apply directly.

* **784. Gray Code (Medium)**
  Direct match with our `gray::binary` and `gray::loopless` implementations ([reddit.com][3], [books.halfrost.com][1]).

---

### 3. **Number of Squareful Arrays (Hard)**

LeetCode **996. Number of Squareful Arrays (Hard)** requires enumerating permutations with a squared-sum constraint. It's a backtracking problem on permutations with pruning — ideal for our permutation & backtracking modules ([books.halfrost.com][1]).

---

### 4. **Unique Paths III (Hard)**

LeetCode **980. Unique Paths III (Hard)** involves traversing all paths exactly once in a grid with preconditions. It's a Hamiltonian path problem solved with constrained DFS — a pattern aligned with our backtracking framework ([books.halfrost.com][1]).

---

### 5. **Distribute Repeating Integers (Hard)**

LeetCode **1655. Distribute Repeating Integers (Hard)** demands generating all assignments of labeled items to buckets under constraints — another variant of backtracking partitioning over labeled elements ([books.halfrost.com][1]).

---

## 📎 Summary Table

|     Problem No. | Title                         |  Difficulty | Technique                     | Rust Module                       |
| --------------: | ----------------------------- | :---------: | ----------------------------- | --------------------------------- |
|              37 | Sudoku Solver                 |     Hard    | Backtracking + constraints    | `backtrack`                       |
|         51 / 52 | N‑Queens / N‑Queens II        |     Hard    | Permutation + pruning         | `backtrack::nqueens`              |
|             212 | Word Search II                |     Hard    | Backtracking on grid/tries    | `backtrack`                       |
|             996 | Number of Squareful Arrays    |     Hard    | Permutations with constraints | `ranking`, `backtrack`            |
|             980 | Unique Paths III              |     Hard    | DFS / Backtracking            | `backtrack`                       |
|            1655 | Distribute Repeating Integers |     Hard    | Partitioning / Backtracking   | `partition_set`, `backtrack`      |
| 47, 90, 17, 784 | Perm/Subset/Gray variants     | Medium/Hard | Permutations, combinations    | `gray`, `combinations`, `minimal` |

---

## ✅ Next Steps

1. **Add problem-specific examples** under `examples/leetcode_*`, showing how to use the crate to solve each problem.
2. **Link solutions in README** under a new **LeetCode Problems** section.
3. Optionally **highlight them in documentation**, with direct references to modules used.

Would you like me to generate example code for one of these LeetCode problems (e.g. Sudoku solver using Rust `backtrack`) next?

[1]: https://books.halfrost.com/leetcode/ChapterTwo/Backtracking/?utm_source=chatgpt.com "2.08 ✅ Backtracking | LeetCode Cookbook"
[2]: https://github.com/tiationg-kho/leetcode-pattern-500/blob/main/README.md?utm_source=chatgpt.com "leetcode-pattern-500/README.md at main · tiationg-kho/leetcode-pattern-500 · GitHub"
[3]: https://www.reddit.com/r/leetcode/comments/1j7gvs4?utm_source=chatgpt.com "Lists of \"problem groups\" i.e problems that go together"

Perfect — I’ll generate Rust examples for each LeetCode problem using our crate’s modules. Each will go into `examples/leetcode_XXX.rs` where `XXX` is the problem number.

---

## 🗂️ Examples to be Generated:

| File               | LeetCode Problem                        | Core Modules Used                      |
| ------------------ | --------------------------------------- | -------------------------------------- |
| `leetcode_037.rs`  | 37. Sudoku Solver                       | `backtrack::constraint` (custom)       |
| `leetcode_051.rs`  | 51. N-Queens                            | `backtrack::nqueens`                   |
| `leetcode_052.rs`  | 52. N-Queens II                         | `backtrack::nqueens`                   |
| `leetcode_212.rs`  | 212. Word Search II                     | `backtrack` (grid trie)                |
| `leetcode_996.rs`  | 996. Squareful Arrays                   | `ranking`, `backtrack::perm_with_pred` |
| `leetcode_980.rs`  | 980. Unique Paths III                   | `backtrack`                            |
| `leetcode_1655.rs` | 1655. Distribute Repeating Integers     | `partition_set`, `backtrack`           |
| `leetcode_784.rs`  | 784. Gray Code                          | `gray::binary`                         |
| `leetcode_047.rs`  | 47. Permutations II                     | `minimal` + deduplication              |
| `leetcode_090.rs`  | 90. Subsets II                          | `combinations::subset_with_dupes`      |
| `leetcode_017.rs`  | 17. Letter Combinations of Phone Number | `backtrack::cartesian_product`         |

---

Would you like me to:

* Begin with **N-Queens** (`leetcode_051.rs`), since it uses our existing module directly?
* Then proceed with the others in order?

Confirm and I’ll begin writing + packaging example 051 now.
