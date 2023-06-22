// https://leetcode.com/problems/best-time-to-buy-and-sell-stock-with-transaction-fee/description/

pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>, fee: i32) -> i32 {
        let (mut not_holding_max, mut holding_max) = (0, -prices[0]);
        for day in 1..prices.len() {
            let old_holding_max = holding_max;
            holding_max = holding_max.max(not_holding_max - prices[day]);
            not_holding_max = not_holding_max.max(old_holding_max + prices[day] - fee);
        }
        not_holding_max.max(holding_max)
    }
}

fn main() {
    println!("Hello, world!");
}
