// https://leetcode.com/problems/maximum-number-of-moves-in-a-grid/description/
pub struct Solution;

impl Solution {
    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![0; grid.len()];
        let mut prev_dp = dp.clone();

        let mut max = i32::MIN;
        for col in 1..grid[0].len() {
            std::mem::swap(&mut dp, &mut prev_dp);
            for row in 0..grid.len() {
                if row > 0 && grid[row][col] > grid[row - 1][col - 1] {
                    dp[row] = dp[row].max(prev_dp[row - 1] + 1);
                }

                if grid[row][col] > grid[row][col - 1] {
                    dp[row] = dp[row].max(prev_dp[row] + 1);
                }

                if row < grid.len() - 1 && grid[row][col] > grid[row + 1][col - 1] {
                    dp[row] = dp[row].max(prev_dp[row + 1] + 1);
                }
            }

            dp.iter_mut().for_each(|moves| {
                if *moves < col as i32 {
                    *moves = 0
                }
                max = max.max(*moves);
            });
        }

        max
    }
}

fn main() {
    println!("Hello, world!");
}
