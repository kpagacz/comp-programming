// https://leetcode.com/problems/final-prices-with-a-special-discount-in-a-shop/description/
pub struct Solution;

impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut discounted = Vec::with_capacity(prices.len());

        for i in 0..prices.len() {
            let mut discount_pos = i + 1;

            while discount_pos < prices.len() && prices[i] < prices[discount_pos] {
                discount_pos += 1;
            }

            if discount_pos == prices.len() {
                discounted.push(prices[i]);
            } else {
                discounted.push(prices[i] - prices[discount_pos]);
            }
        }

        discounted
    }
}

fn main() {
    println!("Hello, world!");
}
