//! LeetCode 037 - Sudoku Solver
//! Example binary: runs a sample puzzle and prints the solved board.

fn is_valid(board: &[Vec<char>], r: usize, c: usize, ch: char) -> bool {
    for i in 0..9 {
        if board[r][i] == ch || board[i][c] == ch {
            return false;
        }
        let br = 3 * (r / 3) + i / 3;
        let bc = 3 * (c / 3) + i % 3;
        if board[br][bc] == ch {
            return false;
        }
    }
    true
}

fn solve_sudoku(board: &mut Vec<Vec<char>>) -> bool {
    for r in 0..9 {
        for c in 0..9 {
            if board[r][c] == '.' {
                for d in '1'..='9' {
                    if is_valid(board, r, c, d) {
                        board[r][c] = d;
                        if solve_sudoku(board) {
                            return true;
                        }
                        board[r][c] = '.';
                    }
                }
                return false;
            }
        }
    }
    true
}

fn print_board(board: &[Vec<char>]) {
    for row in board {
        let s: String = row.iter().collect();
        println!("{}", s);
    }
}

fn main() {
    let mut board = vec![
        vec!['5','3','.','.','7','.','.','.','.'],
        vec!['6','.','.','1','9','5','.','.','.'],
        vec!['.','9','8','.','.','.','.','6','.'],
        vec!['8','.','.','.','6','.','.','.','3'],
        vec!['4','.','.','8','.','3','.','.','1'],
        vec!['7','.','.','.','2','.','.','.','6'],
        vec!['.','6','.','.','.','.','2','8','.'],
        vec!['.','.','.','4','1','9','.','.','5'],
        vec!['.','.','.','.','8','.','.','7','9'],
    ];
    println!("Puzzle:");
    print_board(&board);
    if solve_sudoku(&mut board) {
        println!("\nSolved:");
        print_board(&board);
    } else {
        println!("No solution found.");
    }
}
