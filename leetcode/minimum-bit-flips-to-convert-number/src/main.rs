// https://leetcode.com/problems/minimum-bit-flips-to-convert-number/description/
pub struct Solution;

impl Solution {
    pub fn min_bit_flips(start: i32, goal: i32) -> i32 {
        (start ^ goal).count_ones() as _
    }
}

fn main() {
    println!("Hello, world!");
}
