// https://leetcode.com/problems/number-of-dice-rolls-with-target-sum/description/
pub struct Solution;

const MODULO: i32 = 10i32.pow(9) + 7;
impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        let mut dp = vec![vec![0; target as usize + 1]; n as usize + 1];
        dp[0][0] = 1;
        for dice in 0..dp.len() - 1 {
            for sum in 0..dp[0].len() {
                for i in 1..=k as usize {
                    if sum + i < dp[0].len() {
                        dp[dice + 1][sum + i] = (dp[dice][sum] + dp[dice + 1][sum + i]) % MODULO;
                    }
                }
            }
        }
        dp[n as usize][target as usize]
    }
}

fn main() {
    let test_cases = [(1, 6, 3), (2, 6, 7), (30, 30, 500)];
    for (n, k, target) in test_cases {
        println!("{}", Solution::num_rolls_to_target(n, k, target));
    }
}
