// https://leetcode.com/problems/count-the-number-of-good-subarrays/description/?envType=daily-question&envId=2025-04-16
pub struct Solution;

impl Solution {
    pub fn count_good(nums: Vec<i32>, k: i32) -> i64 {
        use std::collections::HashMap;

        let mut left = 0;
        let mut curr = 0;
        let mut answer = 0;
        let mut freq = HashMap::new();

        for i in 0..nums.len() {
            let entry = freq.entry(nums[i]).or_insert(0);
            *entry += 1;
            curr += *entry - 1;

            while curr >= k && left <= i {
                answer += (nums.len() - i) as i64;
                let entry = freq.entry(nums[left]).or_insert(0);
                *entry -= 1;
                curr -= *entry;
                left += 1;
            }
        }
        answer
    }
}

fn main() {
    println!("Hello, world!");
}
