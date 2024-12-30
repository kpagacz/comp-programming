// https://leetcode.com/problems/count-ways-to-build-good-strings/description/
pub struct Solution;

impl Solution {
    pub fn count_good_strings(low: i32, high: i32, zero: i32, one: i32) -> i32 {
        const MODULO: i32 = 1_000_000_000 + 7;
        let low = low as usize;
        let high = high as usize;
        let zero = zero as usize;
        let one = one as usize;

        let mut dp = vec![0; high + 1];
        dp[0] = 1;

        for i in 0..dp.len() {
            if i + zero < dp.len() {
                dp[i + zero] += dp[i];
                dp[i + zero] %= MODULO;
            }
            if i + one < dp.len() {
                dp[i + one] += dp[i];
                dp[i + one] %= MODULO;
            }
        }

        dp[low..=high]
            .iter()
            .fold(0, |acc, count| (acc + *count) % MODULO)
    }
}

fn main() {
    println!("Hello, world!");
}
