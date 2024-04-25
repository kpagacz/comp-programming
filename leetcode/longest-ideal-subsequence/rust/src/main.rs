// https://leetcode.com/problems/longest-ideal-subsequence/description/
pub struct Solution;

impl Solution {
    pub fn longest_ideal_string(s: String, k: i32) -> i32 {
        let mut dp = [0; 26];

        let s = s.as_bytes().iter().map(|b| b - b'a');
        let k = k as usize;

        for c in s {
            let c = c as usize;
            let diff_start = c.saturating_sub(k);
            let diff_end = (c + k).min(25);
            let new_longest = dp[diff_start..=diff_end]
                .iter()
                .map(|longest| longest + 1)
                .max()
                .unwrap();
            dp[c] = new_longest;
        }

        *dp.iter().max().unwrap()
    }
}

fn main() {
    println!("Hello, world!");
}
