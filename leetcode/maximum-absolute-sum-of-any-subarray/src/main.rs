// https://leetcode.com/problems/maximum-absolute-sum-of-any-subarray/description/
pub struct Solution;

impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut max_sum = 0;
        let mut current_sum = 0;

        for &x in nums.iter() {
            current_sum = x.max(current_sum + x);
            max_sum = max_sum.max(current_sum);
        }

        current_sum = 0;
        for x in nums {
            current_sum = x.min(current_sum + x);
            max_sum = max_sum.max(current_sum.abs());
        }

        max_sum
    }
}

fn main() {
    println!("Hello, world!");
}
