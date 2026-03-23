// https://leetcode.com/problems/maximum-non-negative-product-in-a-matrix/description/?envType=daily-question&envId=2026-03-23
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![(i64::MAX, i64::MIN); n]; m];
        dp[m - 1][n - 1] = (grid[m - 1][n - 1] as i64, grid[m - 1][n - 1] as i64);
        const MOD: i64 = 10i64.pow(9) + 7;

        for row in (0..grid.len()).rev() {
            for col in (0..grid[0].len()).rev() {
                if row < m - 1 {
                    dp[row][col].0 = dp[row][col]
                        .0
                        .min(dp[row + 1][col].0 * grid[row][col] as i64)
                        .min(dp[row + 1][col].1 * grid[row][col] as i64);
                    dp[row][col].1 = dp[row][col]
                        .1
                        .max(dp[row + 1][col].0 * grid[row][col] as i64)
                        .max(dp[row + 1][col].1 * grid[row][col] as i64);
                }
                if col < n - 1 {
                    dp[row][col].0 = dp[row][col]
                        .0
                        .min(dp[row][col + 1].0 * grid[row][col] as i64)
                        .min(dp[row][col + 1].1 * grid[row][col] as i64);
                    dp[row][col].1 = dp[row][col]
                        .1
                        .max(dp[row][col + 1].0 * grid[row][col] as i64)
                        .max(dp[row][col + 1].1 * grid[row][col] as i64);
                }
            }
        }

        if dp[0][0].1 < 0 {
            -1
        } else {
            (dp[0][0].1 % MOD) as _
        }
    }
}

fn main() {
    let test_cases = [(
        vec![
            vec![1, -1, 2, 1, -1, 0, 0, 4, 3, 2, 0, -2, -2],
            vec![-2, 3, 3, -1, -1, 0, 0, -2, 4, -3, 3, 0, 0],
            vec![-4, -1, -1, -2, 2, -1, -2, -2, 0, 3, -1, -4, 1],
            vec![-3, 4, -3, 0, -3, 1, -3, 1, 4, 4, -4, -4, -2],
            vec![3, -3, 1, 0, -1, -4, -4, -4, 3, 2, 2, 3, 3],
            vec![2, -1, -1, -4, -3, -3, 4, 2, 3, 4, 4, -4, 0],
            vec![4, -1, 2, -3, -1, -1, -3, -4, 4, 4, 4, -3, -1],
            vec![-3, -4, 4, -2, -1, 2, 3, -1, 2, 3, 4, 4, -4],
            vec![-3, -1, -2, 1, 1, -1, -3, -4, -3, 1, -3, 3, -4],
            vec![2, 4, 4, 4, -3, -3, 1, -1, 3, 4, -1, 1, 4],
            vec![2, -2, 0, 4, -1, 0, -2, 4, -4, 0, 0, 2, -3],
            vec![1, 1, -3, 0, -4, -4, -4, -4, 0, -1, -4, -1, 0],
            vec![3, -1, -3, -3, -3, -2, -1, 4, -1, -2, 4, 2, 3],
        ],
        459630706,
    )];
    for (grid, exp) in test_cases {
        println!("{} exp: {exp}", Solution::max_product_path(grid));
    }
}
