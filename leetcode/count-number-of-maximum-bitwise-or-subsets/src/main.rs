// https://leetcode.com/problems/count-number-of-maximum-bitwise-or-subsets/description/
pub struct Solution;

impl Solution {
    pub fn count_max_or_subsets(nums: Vec<i32>) -> i32 {
        let max_or = nums.iter().fold(0, |max_or, num| max_or | *num);

        fn backtrack(nums_pos: usize, current: i32, target: i32, nums: &[i32]) -> i32 {
            if nums_pos == nums.len() {
                if current == target {
                    return 1;
                } else {
                    return 0;
                }
            }

            backtrack(nums_pos + 1, current, target, nums)
                + backtrack(nums_pos + 1, current | nums[nums_pos], target, nums)
        }

        backtrack(0, 0, max_or, &nums)
    }
}

fn main() {
    println!("Hello, world!");
}
