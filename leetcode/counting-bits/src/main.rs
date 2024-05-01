// https://leetcode.com/problems/counting-bits/description/
struct Solution;

impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut dp = vec![0; n + 1];

        for i in 0..=n {
            dp[i] = dp[i >> 1] as i32 + (i % 2) as i32;
        }

        dp
    }
}

fn main() {
    println!("Hello, world!");
}
