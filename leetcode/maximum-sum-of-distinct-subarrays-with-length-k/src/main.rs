// https://leetcode.com/problems/maximum-sum-of-distinct-subarrays-with-length-k/
pub struct Solution;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        use std::collections::HashMap;
        let mut curr = 0;
        let mut max = 0;
        let mut freq: HashMap<i32, i32> = HashMap::default();
        let mut left = 0;
        let k = k as usize;
        for i in 0..nums.len() {
            *freq.entry(nums[i]).or_insert(0) += 1;
            curr += nums[i] as i64;

            while (*freq.get(&nums[i]).unwrap() > 1 && left < i) || (i + 1 - left) > k {
                *freq.entry(nums[left]).or_insert(0) -= 1;
                curr -= nums[left] as i64;
                left += 1;
            }

            if i + 1 - left == k {
                max = max.max(curr);
            }
        }
        max
    }
}

fn main() {
    println!("Hello, world!");
}
