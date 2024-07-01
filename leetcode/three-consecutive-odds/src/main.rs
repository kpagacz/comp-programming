// https://leetcode.com/problems/three-consecutive-odds/description/
pub struct Solution;

impl Solution {
    pub fn three_consecutive_odds(arr: Vec<i32>) -> bool {
        arr.windows(3).any(|window| {
            window[0] % 2 == 1 && window[0] % 2 == window[1] % 2 && window[1] % 2 == window[2] % 2
        })
    }
}

fn main() {
    println!("Hello, world!");
}
