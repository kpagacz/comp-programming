
// https://leetcode.com/problems/largest-submatrix-with-rearrangements/
pub struct Solution {}

use std::cmp::Reverse;
impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        // dp[i][j] is the number of consecutive ones in the column j ending at row i
        let mut dp = vec![vec![0; matrix[0].len()]; matrix.len()];

        for col in 0..matrix[0].len() {
            let mut ones = 0;
            for row in 0..matrix.len() {
                if matrix[row][col] == 1 {
                    ones += 1;
                } else {
                    ones = 0;
                }
                dp[row][col] = ones;
            }
        }
        dp.iter_mut()
            .for_each(|row| row.sort_unstable_by_key(|el| Reverse(*el)));
        println!("{dp:?}");

        let mut max = 0;
        for row in &dp {
            let mut cols = 0;
            let mut min_ones = i32::MAX;
            for &previous_ones in row {
                cols += 1;
                min_ones = min_ones.min(previous_ones);
                max = max.max(cols * min_ones);
            }
        }
        max
    }
}
fn main() {
    let test_cases = vec![vec![vec![0, 0, 1], vec![1, 1, 1], vec![1, 0, 1]]];
    for matrix in test_cases {
        println!("{}", Solution::largest_submatrix(matrix));
    }
}
