// https://leetcode.com/problems/number-of-1-bits/description/
pub struct Solution {}

impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        n.count_ones() as i32
    }
}

fn main() {
    println!("Hello, world!");
}
