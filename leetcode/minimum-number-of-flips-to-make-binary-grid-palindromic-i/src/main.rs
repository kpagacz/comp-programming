// https://leetcode.com/problems/minimum-number-of-flips-to-make-binary-grid-palindromic-i/description/
pub struct Solution;

impl Solution {
    pub fn min_flips(grid: Vec<Vec<i32>>) -> i32 {
        let mut rows_errors = 0;
        let mut cols_errors = 0;

        grid.iter().for_each(|row| {
            let n = row.len();
            for i in 0..row.len() / 2 {
                if row[i] != row[n - i - 1] {
                    rows_errors += 1;
                }
            }
        });

        for col in 0..grid[0].len() {
            for row in 0..grid.len() / 2 {
                if grid[row][col] != grid[grid.len() - 1 - row][col] {
                    cols_errors += 1;
                }
            }
        }

        rows_errors.min(cols_errors)
    }
}

fn main() {
    println!("Hello, world!");
}
