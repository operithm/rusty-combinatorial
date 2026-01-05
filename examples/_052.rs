//! LeetCode 51: N-Queens
//! Solves the classic n-Queens problem using our backtrack::nqueens module

use rusty_combinatorial::backtrack::nqueens::solve_nqueens;

/// Converts a solution vector into the expected LeetCode board format
fn to_board_format(sol: &[usize]) -> Vec<String> {
    let n = sol.len();
    sol.iter()
        .map(|&col| {
            let mut row = vec!['.'; n];
            row[col] = 'Q';
            row.iter().collect()
        })
        .collect()
}

fn solve_n_queens(n: usize) -> Vec<Vec<String>> {
    solve_nqueens(n)
        .iter()
        .map(|sol| to_board_format(sol))
        .collect()
}

/// LeetCode 52: N-Queens II
/// Returns only the number of valid solutions for n-Queens
fn total_n_queens(n: usize) -> i32 {
    solve_nqueens(n).len() as i32
}

fn main() {
    println!("Solutions for LeetCode 51 (N-Queens):");
    let boards = solve_n_queens(4);
    for board in boards {
        for row in board.iter() {
            println!("{}", row);
        }
        println!();
    }

    println!("Total solutions for LeetCode 52 (N-Queens II): {}", total_n_queens(8));
}
