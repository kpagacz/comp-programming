// https://leetcode.com/problems/maximum-average-subarray-i/description/
pub struct Solution;

impl Solution {
    pub fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
        let k = k as usize;

        let mut curr = nums[0..k].iter().sum::<i32>();
        let mut max = curr;
        for i in k..nums.len() {
            curr -= nums[i - k];
            curr += nums[i];
            max = i32::max(max, curr);
        }

        max as f64 / k as f64
    }
}

fn main() {
    println!("Hello, world!");
}
