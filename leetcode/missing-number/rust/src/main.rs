// https://leetcode.com/problems/missing-number/description/
pub struct Solution;

impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        n * (n + 1) / 2 - nums.iter().sum::<i32>()
    }
}

fn main() {
    println!("Hello, world!");
}
