// https://leetcode.com/problems/maximum-width-ramp/description/
pub struct Solution;

impl Solution {
    pub fn max_width_ramp(nums: Vec<i32>) -> i32 {
        let mut monotonic_stack = vec![];

        for pos in 0..nums.len() {
            if monotonic_stack.is_empty() || nums[pos] < nums[*monotonic_stack.last().unwrap()] {
                monotonic_stack.push(pos);
            }
        }

        let mut max_width = 0;

        for (pos, num) in nums.iter().enumerate().rev() {
            while !monotonic_stack.is_empty() && num >= &nums[*monotonic_stack.last().unwrap()] {
                max_width = max_width.max(pos - *monotonic_stack.last().unwrap());
                monotonic_stack.pop();
            }
        }

        max_width as _
    }
}

fn main() {
    println!("Hello, world!");
}
