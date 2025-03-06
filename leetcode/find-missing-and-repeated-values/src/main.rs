// https://leetcode.com/problems/find-missing-and-repeated-values/description/
pub struct Solution;

impl Solution {
    pub fn find_missing_and_repeated_values(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let n = grid.len() as i32;
        let expected_sum = (1..=n * n).sum::<i32>();
        let expected_ssum = (1..=n * n).fold(0, |sum, elem| sum + elem * elem);
        let mut actual_sum = 0;
        let mut actual_ssum = 0;

        for i in 0..n as usize {
            for j in 0..n as usize {
                actual_sum += grid[i][j];
                actual_ssum += grid[i][j] * grid[i][j];
            }
        }

        let a_minus_b = actual_sum - expected_sum;
        let a_plus_b = (actual_ssum - expected_ssum) / a_minus_b;

        vec![(a_plus_b + a_minus_b) / 2, (a_plus_b - a_minus_b) / 2]
    }
}

fn main() {
    println!("Hello, world!");
}
