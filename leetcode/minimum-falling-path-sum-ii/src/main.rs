// https://leetcode.com/problems/minimum-falling-path-sum-ii/description/
pub struct Solution;

impl Solution {
    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![0; grid.len()];

        for row in &grid {
            let mut new_dp = vec![0; grid.len()];

            let min_pos = Solution::min_position(&dp);
            let second_lowest = Solution::min_without_position(&dp, min_pos);

            for i in 0..row.len() {
                if i != min_pos {
                    new_dp[i] = row[i] + dp[min_pos];
                } else {
                    new_dp[i] = row[i] + second_lowest;
                }
            }

            dp = new_dp;
        }

        *dp.iter().min().unwrap()
    }

    fn min_position(row: &[i32]) -> usize {
        row.iter()
            .enumerate()
            .min_by_key(|(_, &val)| val)
            .map(|(pos, _)| pos)
            .unwrap()
    }

    fn min_without_position(row: &[i32], pos: usize) -> i32 {
        let max = *row.iter().max().unwrap();
        row.iter().enumerate().fold(
            max,
            |min, (id, val)| if pos != id { min.min(*val) } else { min },
        )
    }
}

fn main() {
    println!("Hello, world!");
}
