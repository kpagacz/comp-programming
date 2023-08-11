// https://leetcode.com/problems/coin-change-ii/
pub struct Solution {}

impl Solution {
    // Runtime 1ms Beats 78.57%of users with Rust
    // Memory 2.04mb Beats 64.29%of users with Rust
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let n = (amount + 1) as usize;
        let mut dp = vec![0; n];
        dp[0] = 1;
        for coin in &coins {
            let coin_usize = *coin as usize;
            for i in 0..n {
                if coin_usize + i < n {
                    dp[coin_usize + i] += dp[i];
                }
            }
        }
        dp[n - 1]
    }
}

fn main() {
    let test_cases = vec![(5, vec![1, 2, 5])];

    for (amount, coins) in test_cases {
        Solution::change(amount, coins);
    }
}
