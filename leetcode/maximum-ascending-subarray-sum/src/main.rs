// https://leetcode.com/problems/maximum-ascending-subarray-sum/
pub struct Solution;

impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        nums.chunk_by(|a, b| a < b)
            .map(|chunk| chunk.iter().sum::<i32>())
            .max()
            .unwrap()
    }
}

fn main() {
    println!("Hello, world!");
}
