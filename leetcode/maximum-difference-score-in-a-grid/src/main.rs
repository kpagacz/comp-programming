// https://leetcode.com/problems/maximum-difference-score-in-a-grid/description/
pub struct Solution;

impl Solution {
    pub fn max_score(grid: Vec<Vec<i32>>) -> i32 {
        let (mut dp, mut answer) = (grid.clone(), i32::MIN);

        for row in 0..grid.len() {
            for col in 0..grid[0].len() {
                if row > 0 {
                    answer = answer.max(grid[row][col] - dp[row - 1][col]);
                    dp[row][col] = dp[row][col].min(dp[row - 1][col]);
                }
                if col > 0 {
                    answer = answer.max(grid[row][col] - dp[row][col - 1]);
                    dp[row][col] = dp[row][col].min(dp[row][col - 1]);
                }
            }
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
