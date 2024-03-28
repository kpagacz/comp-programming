// https://leetcode.com/problems/length-of-longest-subarray-with-at-most-k-frequency/description/
pub struct Solution;

impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut left = 0;
        let mut max_length = 0;
        use std::collections::HashMap;
        let mut freqs = HashMap::new();

        for i in 0..nums.len() {
            *freqs.entry(nums[i]).or_insert(0) += 1;
            while *freqs.get(&nums[i]).unwrap() > k {
                *freqs.entry(nums[left]).or_insert(0) -= 1;
                left += 1;
            }
            max_length = max_length.max(i - left + 1);
        }

        max_length as _
    }
}

fn main() {
    println!("Hello, world!");
}
