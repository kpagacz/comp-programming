// https://leetcode.com/problems/number-of-smooth-descent-periods-of-a-stock/description/?envType=daily-question&envId=2025-12-15
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let mut answer = 1;
        let mut start = 0usize;
        for i in 1..prices.len() {
            if prices[i - 1] - prices[i] != 1 {
                start = i;
                answer += 1;
            } else {
                answer += i - start + 1;
            }
        }
        answer as _
    }
}

fn main() {
    println!("Hello, world!");
}
