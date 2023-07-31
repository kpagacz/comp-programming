// https://leetcode.com/problems/minimum-ascii-delete-sum-for-two-strings/
pub struct Solution {}

impl Solution {
    pub fn minimum_delete_sum(s1: String, s2: String) -> i32 {
        let mut dp = vec![vec![-1; s2.len() + 1]; s1.len() + 1];
        let mut s1_sum = 0;
        dp[s1.len()][s2.len()] = 0;
        for i in 0..s1.len() {
            s1_sum += s1.as_bytes()[s1.len() - 1 - i] as i32;
            dp[s1.len() - 1 -i][s2.len()] = s1_sum;
        }
        let mut s2_sum = 0;
        for i in 0..s2.len() {
            s2_sum += s2.as_bytes()[s2.len() - 1 - i] as i32;
            dp[s1.len()][s2.len() - 1 - i] = s2_sum;
        }

        for i in (0..s1.len()).rev() {
            for j in (0..s2.len()).rev() {
                let i_char = s1.as_bytes()[i];
                let j_char = s2.as_bytes()[j];
                if i_char != j_char {
                    dp[i][j] =
                        std::cmp::min(dp[i + 1][j] + i_char as i32, dp[i][j + 1] + j_char as i32);
                } else {
                    dp[i][j] = dp[i + 1][j + 1];
                }
            }
        }

        dp[0][0]
    }
}

fn main() {
    println!("Hello, world!");
}

[[406, 312, 413, 514, 627] d
 [306, 212, 313, 414, 519] e
 [205, 313, 212, 313, 418] l
 [313, 205, 104, 205, 317] e
 [414, 306, 205, 104, 201] t
 [325, 217, 116, 217, 104] e
 [426, 318, 217, 116, 0]]  0
    l   e    e    t   0
