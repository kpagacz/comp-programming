// https://leetcode.com/problems/maximum-product-of-three-numbers/description/
pub struct Solution;

impl Solution {
    pub fn maximum_product(mut nums: Vec<i32>) -> i32 {
        use std::cmp::Reverse;
        nums.sort_unstable_by_key(|k| Reverse(*k));

        (nums[0] * nums[1] * nums[2]).max(nums[nums.len() - 1] * nums[nums.len() - 2] * nums[0])
    }
}

fn main() {
    println!("Hello, world!");
}
