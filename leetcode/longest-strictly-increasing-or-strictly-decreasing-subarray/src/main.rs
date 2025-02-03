// https://leetcode.com/problems/longest-strictly-increasing-or-strictly-decreasing-subarray/description/
pub struct Solution;

impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        nums.chunk_by(|a, b| a > b)
            .chain(nums.chunk_by(|a, b| b > a))
            .fold(0, |m, l| m.max(l.len() as _))
    }
}

fn main() {
    println!("Hello, world!");
}
