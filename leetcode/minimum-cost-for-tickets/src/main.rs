// https://leetcode.com/problems/minimum-cost-for-tickets/description/
pub struct Solution;

impl Solution {
    pub fn mincost_tickets(days: Vec<i32>, costs: Vec<i32>) -> i32 {
        let mut dp = [0; 400];

        for i in (0..366).rev() {
            if days.binary_search(&(i as i32)).is_ok() {
                dp[i] = (dp[i + 1] + costs[0])
                    .min(dp[i + 7] + costs[1])
                    .min(dp[i + 30] + costs[2]);
            } else {
                dp[i] = dp[i + 1];
            }
        }

        dp[0]
    }
}

fn main() {
    println!("Hello, world!");
}
