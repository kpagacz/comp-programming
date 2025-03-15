// https://leetcode.com/problems/house-robber-iv/description/?envType=daily-question&envId=2025-03-15
pub struct Solution;

impl Solution {
    pub fn min_capability(nums: Vec<i32>, k: i32) -> i32 {
        fn is_possible(nums: &[i32], mut k: i32, limit: i32) -> bool {
            let mut it = 0;
            while it < nums.len() && k > 0 {
                if nums[it] <= limit {
                    k -= 1;
                    it += 2;
                } else {
                    it += 1;
                }
            }
            k == 0
        }

        let mut left = *nums.iter().min().unwrap();
        let mut right = *nums.iter().max().unwrap();

        while left < right {
            let middle = left + (right - left) / 2;

            if is_possible(&nums, k, middle) {
                right = middle;
            } else {
                left = middle + 1;
            }
        }
        left
    }
}

fn main() {
    println!("Hello, world!");
}
