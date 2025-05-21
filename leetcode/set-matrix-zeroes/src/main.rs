// https://leetcode.com/problems/set-matrix-zeroes/description/
pub struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut [Vec<i32>]) {
        let rows = matrix.len();
        let cols = matrix[0].len();
        let first_row_zeros = matrix[0].iter().any(|val| *val == 0);
        let first_col_zeros = matrix.iter().map(|row| row[0]).any(|val| val == 0);
        for row in 1..rows {
            for col in 1..cols {
                if matrix[row][col] == 0 {
                    matrix[row][0] = 0;
                    matrix[0][col] = 0;
                }
            }
        }
        (1..rows).for_each(|row| {
            if matrix[row][0] == 0 {
                for col in 0..cols {
                    matrix[row][col] = 0;
                }
            }
        });
        for col in 0..cols {
            if matrix[0][col] == 0 {
                (0..rows).for_each(|row| {
                    matrix[row][col] = 0;
                });
            }
        }
        if first_row_zeros {
            matrix[0].fill(0);
        }
        if first_col_zeros {
            (0..rows).for_each(|row| {
                matrix[row][0] = 0;
            });
        }
    }
}

fn main() {
    println!("Hello, world!");
}
