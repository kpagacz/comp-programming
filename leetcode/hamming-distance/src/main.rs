// https://leetcode.com/problems/hamming-distance/description/
pub struct Solution;

impl Solution {
    pub fn hamming_distance(x: i32, y: i32) -> i32 {
        (x ^ y).count_ones() as _
    }
}

fn main() {
    println!("Hello, world!");
}
