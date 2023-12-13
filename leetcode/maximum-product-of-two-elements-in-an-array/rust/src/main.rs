// https://leetcode.com/problems/maximum-product-of-two-elements-in-an-array/description/?envType=daily-question&envId=2023-12-12
pub struct Solution {}

impl Solution {
    pub fn max_product(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        (nums[nums.len() - 1] - 1) * (nums[nums.len() - 2] - 1)
    }
}

fn main() {
    println!("Hello, world!");
}
