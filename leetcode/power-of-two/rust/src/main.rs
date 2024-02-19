// https://leetcode.com/problems/power-of-two/
pub struct Solution;

impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n.count_ones() == 1 && n > 0
    }
}

fn main() {
    println!("Hello, world!");
}
