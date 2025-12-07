// https://leetcode.com/problems/count-odd-numbers-in-an-interval-range/description/?envType=daily-question&envId=2025-12-07
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        (high - low + 1) / 2 + (if high % 2 == 1 && low % 2 == 1 { 1 } else { 0 })
    }
}
fn main() {
    println!("Hello, world!");
}
