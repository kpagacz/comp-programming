// https://leetcode.com/problems/maximum-product-difference-between-two-pairs/
pub struct Solution {}

impl Solution {
    pub fn max_product_difference(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums[nums.len() - 1] * nums[nums.len() - 2] - nums[0] * nums[1]
    }
}

fn main() {
    println!("Hello, world!");
}
