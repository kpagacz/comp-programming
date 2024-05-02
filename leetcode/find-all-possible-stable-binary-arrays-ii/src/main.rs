// https://leetcode.com/problems/find-all-possible-stable-binary-arrays-ii/description/
pub struct Solution;

impl Solution {
    pub fn number_of_stable_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        let (zero, one, limit) = (zero as usize, one as usize, limit as usize);
        let mut dp = vec![vec![vec![0; 2]; one + 1]; zero + 1];
        // dp[zero][one][0/1] the number of stable
        // arrays with zero 0s one 1s and ending on 0 or 1
        let modulo = 10i32.pow(9) + 7;

        for i in 0..=zero {
            for j in 0..=one {
                if i == 0 || j == 0 {
                    // basecase where the array is full of
                    // either 1s or 0s
                    if i > 0 && i <= limit {
                        dp[i][j][0] = 1;
                    }
                    if j > 0 && j <= limit {
                        dp[i][j][1] = 1;
                    }
                    continue;
                }

                if i > 0 {
                    // just adding a 0
                    dp[i][j][0] = (dp[i - 1][j][0] + dp[i - 1][j][1]) % modulo;
                }
                if i > limit {
                    // subtracting arrays that have a suffix span of limit + 1 0s
                    dp[i][j][0] = (dp[i][j][0] + modulo - dp[i - limit - 1][j][1]) % modulo;
                }

                if j > 0 {
                    // just adding a 1
                    dp[i][j][1] = (dp[i][j - 1][0] + dp[i][j - 1][1]) % modulo;
                }
                if j > limit {
                    // subtracting arrays that have a suffix span of limit + 1 1s
                    dp[i][j][1] = (dp[i][j][1] + modulo - dp[i][j - limit - 1][0]) % modulo;
                }
            }
        }

        (dp[zero][one][0] + dp[zero][one][1]) % modulo
    }
}

fn main() {
    println!("Hello, world!");
}
