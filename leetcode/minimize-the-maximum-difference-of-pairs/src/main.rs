// https://leetcode.com/problems/minimize-the-maximum-difference-of-pairs/description/?envType=daily-question&envId=2025-06-13
pub struct Solution;

impl Solution {
    pub fn minimize_max(mut nums: Vec<i32>, p: i32) -> i32 {
        nums.sort_unstable();
        fn p_pairs_lower_than(nums: &[i32], threshold: u32, p: i32) -> bool {
            let mut it = 1usize;
            let mut count = 0;
            while it < nums.len() {
                if nums[it].abs_diff(nums[it - 1]) <= threshold {
                    it += 2;
                    count += 1;
                } else {
                    it += 1;
                }
            }
            count >= p
        }

        let mut left = 0u32;
        let mut right = (nums.last().unwrap() - nums.first().unwrap()) as u32;
        while left < right {
            let middle = left + (right - left) / 2;

            if p_pairs_lower_than(&nums, middle, p) {
                right = middle;
            } else {
                left = middle + 1;
            }
        }

        left as i32
    }
}

fn main() {
    println!("Hello, world!");
}
