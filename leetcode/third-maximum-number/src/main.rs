// https://leetcode.com/problems/third-maximum-number/description/
pub struct Solution;

impl Solution {
    pub fn third_max(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.dedup();
        if nums.len() < 3 {
            *nums.iter().max().unwrap()
        } else {
            nums[nums.len() - 3]
        }
    }
}

fn main() {
    println!("Hello, world!");
}
