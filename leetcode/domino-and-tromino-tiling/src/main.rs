// https://leetcode.com/problems/domino-and-tromino-tiling/description/?envType=daily-question&envId=2025-05-05
pub struct Solution;

impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        const MOD: i32 = 10i32.pow(9) + 7;
        let n = n as usize;
        let mut dp = vec![vec![0; 3]; n + 4];
        dp[0][0] = 1;

        for i in 0..n {
            // |
            dp[i + 1][0] += dp[i][0];
            dp[i + 1][0] %= MOD;
            // __
            // __
            dp[i + 2][0] += dp[i][0];
            dp[i + 2][0] %= MOD;

            for k in 1..3 {
                // Add one tromino
                // Either it sticks top or bottom
                dp[i + 2][k] += dp[i][0];
                dp[i + 2][k] %= MOD;

                // Finish the sticking out patterns
                dp[i + 1][0] += dp[i][k];
                dp[i + 1][0] %= MOD;
            }
            // Can add one tromino to the sticking out parts
            dp[i + 1][1] += dp[i][2];
            dp[i + 1][1] %= MOD;
            dp[i + 1][2] += dp[i][1];
            dp[i + 1][2] %= MOD;
        }

        dp[n][0]
    }
}

fn main() {
    let test_cases = [(3, 5), (1, 1), (4, 11)];
    for (n, exp) in test_cases {
        println!("{} exp: {exp}", Solution::num_tilings(n));
    }
}
