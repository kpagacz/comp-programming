// https://leetcode.com/problems/sudoku-solver/description/
struct Solution;

const EMPTY: char = '.';
#[allow(dead_code)]
impl Solution {
    pub fn solve_sudoku(board: &mut [Vec<char>]) {
        Solution::rec_solve(board, 0, 0);
    }

    fn validate_last_added(board: &[Vec<char>], row: usize, col: usize) -> bool {
        let last_added = board[row][col];
        let mut count = 0;
        // Row
        for tested_col in 0..board[0].len() {
            if board[row][tested_col] == last_added {
                count += 1;
            }
        }
        if count >= 2 {
            return false;
        }

        // Col
        count = 0;
        for tested_row in 0..board.len() {
            if board[tested_row][col] == last_added {
                count += 1;
            }
        }
        if count >= 2 {
            return false;
        }

        // Small square
        count = 0;
        let row_offset = row / 3;
        let col_offset = col / 3;
        let row_offset = 3 * row_offset;
        let col_offset = 3 * col_offset;

        for dx in 0..3 {
            for dy in 0..3 {
                if board[row_offset + dx][col_offset + dy] == last_added {
                    count += 1;
                }
            }
        }

        count < 2
    }

    fn rec_solve(board: &mut [Vec<char>], current_row: usize, current_col: usize) -> bool {
        if current_row == board.len() {
            return true;
        }

        if board[current_row][current_col] != EMPTY {
            let (new_row, new_col) = Solution::next_position(board, current_row, current_col);
            Solution::rec_solve(board, new_row, new_col)
        } else {
            for inserted in '1'..='9' {
                board[current_row][current_col] = inserted;
                if Solution::validate_last_added(board, current_row, current_col) {
                    let (new_row, new_col) =
                        Solution::next_position(board, current_row, current_col);
                    if Solution::rec_solve(board, new_row, new_col) {
                        return true;
                    }
                }
            }
            board[current_row][current_col] = EMPTY;
            false
        }
    }

    fn next_position(board: &[Vec<char>], row: usize, col: usize) -> (usize, usize) {
        let new_row = row + if col == board[0].len() - 1 { 1 } else { 0 };
        let new_col = (col + 1) % board[0].len();
        (new_row, new_col)
    }
}

fn main() {
    println!("Hello, world!");
}
