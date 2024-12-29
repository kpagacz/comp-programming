// https://leetcode.com/problems/number-of-ways-to-form-a-target-string-given-a-dictionary/description/
pub struct Solution;

impl Solution {
    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        const MODULO: i32 = 1_000_000_000 + 7;
        let word_length = words[0].len();
        let mut chars = vec![vec![0; 256]; word_length];
        words.into_iter().for_each(|word| {
            for (pos, c) in word.chars().enumerate() {
                chars[pos][c as usize] += 1;
            }
        });
        // dp[i][j] - number of ways you can create i chars
        // using j length of words
        let mut dp = vec![vec![0; word_length + 1]; target.len() + 1];
        for i in 0..=word_length {
            dp[0][i] = 1;
        }

        let target: Vec<char> = target.chars().collect();
        for target_c in 1..=target.len() {
            let to_find = target[target_c - 1] as usize;
            for used_c in 1..=word_length {
                dp[target_c][used_c] = dp[target_c][used_c - 1];
                dp[target_c][used_c] += ((chars[used_c - 1][to_find] as i64
                    * dp[target_c - 1][used_c - 1] as i64)
                    % MODULO as i64) as i32;
                dp[target_c][used_c] %= MODULO;
            }
        }

        dp[target.len()][word_length]
    }
}

fn main() {
    println!("Hello, world!");
}
