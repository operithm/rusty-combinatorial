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

fn main() {
    let boards = solve_n_queens(4);
    for board in boards {
        for row in board.iter() {
            println!("{}", row);
        }
        println!();
    }
}
