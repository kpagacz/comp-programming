// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-using-strategy/description/?envType=daily-question&envId=2025-12-18
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_profit(prices: Vec<i32>, strategy: Vec<i32>, k: i32) -> i64 {
        let mut dp = vec![(0i64, 0i64); prices.len()];
        let k = k as usize;
        let mut alternative_strategy_profit = 0i64;

        for day in 0..prices.len() {
            let day_profit = strategy[day] as i64 * prices[day] as i64;
            dp[day].0 = day_profit;
            dp[day].1 = day_profit;
            if day > 0 {
                dp[day].0 += dp[day - 1].0;
                dp[day].1 += dp[day - 1].1;
            }

            alternative_strategy_profit += prices[day] as i64;
            if day >= k / 2 {
                alternative_strategy_profit -= prices[day - k / 2] as i64;
            }

            if day >= k - 1 {
                let before_change = if day >= k { dp[day - k].0 } else { 0 };
                dp[day].1 = dp[day].1.max(before_change + alternative_strategy_profit);
            }
        }

        dp[prices.len() - 1].0.max(dp[prices.len() - 1].1)
    }
}

fn main() {
    let test_cases = vec![
        (vec![4, 2, 8], vec![-1, 0, 1], 2, 10),
        (vec![5, 4, 3], vec![1, 1, 0], 2, 9),
        (vec![4, 7, 13], vec![-1, -1, 0], 2, 9),
    ];
    for (prices, strategy, k, exp) in test_cases {
        println!("{}, exp {exp}", Solution::max_profit(prices, strategy, k));
    }
}
