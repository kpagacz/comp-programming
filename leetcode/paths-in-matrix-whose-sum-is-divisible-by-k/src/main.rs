// https://leetcode.com/problems/paths-in-matrix-whose-sum-is-divisible-by-k/description/?envType=daily-question&envId=2025-11-26
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![vec![0; k as usize]; n]; m];
        let first_num = (grid[0][0] as usize) % k as usize;
        dp[0][0][first_num] = 1;
        const MOD: i32 = 10i32.pow(9) + 7;

        for row in 0..m {
            for col in 0..n {
                let num = (grid[row][col] % k) as usize;
                for rest in 0..k as usize {
                    let previous_rest = (rest + k as usize - num) % k as usize;
                    if row > 0 {
                        dp[row][col][rest] += dp[row - 1][col][previous_rest];
                        dp[row][col][rest] %= MOD;
                    }
                    if col > 0 {
                        dp[row][col][rest] += dp[row][col - 1][previous_rest];
                        dp[row][col][rest] %= MOD;
                    }
                }
            }
        }

        dp[m - 1][n - 1][0]
    }
}

fn main() {
    let test_cases = [(vec![vec![5, 2, 4], vec![3, 0, 5], vec![0, 7, 2]], 3, 2)];

    for (grid, k, exp) in test_cases {
        println!("{}   exp: {exp}", Solution::number_of_paths(grid, k));
    }
}
