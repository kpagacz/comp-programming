// https://leetcode.com/problems/string-compression-ii/description/
pub struct Solution;

impl Solution {
    fn rle_length(s: usize) -> i32 {
        if s >= 100 {
            3
        } else if s >= 10 {
            2
        } else if s >= 2 {
            1
        } else {
            0
        }
    }

    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        let mut dp = vec![vec![9999; k as usize + 1]; s.len() + 1];
        dp[0][0] = 0;

        let s = s.chars().collect::<Vec<_>>();
        for c in 1..dp.len() {
            for dels_left in 0..dp[0].len() {
                // Try to make a run ending at c - 1
                let mut same = 0;
                let mut dels_required = 0;
                for i in (1..=c).rev() {
                    if s[i - 1] == s[c - 1] {
                        same += 1;
                    } else {
                        dels_required += 1;
                    }

                    if dels_left >= dels_required {
                        dp[c][dels_left] = dp[c][dels_left].min(
                            dp[i - 1][dels_left - dels_required] + 1 + Solution::rle_length(same),
                        );
                    }
                }
                // Just delete the character
                if dels_left > 0 {
                    dp[c][dels_left] = dp[c][dels_left].min(dp[c - 1][dels_left - 1]);
                }
            }
        }
        dp[s.len()][k as usize]
    }
}

fn main() {
    let test_cases = [("aaabcccd", 2)];
    for (s, k) in test_cases {
        println!(
            "{}",
            Solution::get_length_of_optimal_compression(s.to_string(), k)
        );
    }
}
