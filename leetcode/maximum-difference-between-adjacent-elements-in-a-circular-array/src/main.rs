// https://leetcode.com/problems/maximum-difference-between-adjacent-elements-in-a-circular-array/description/?envType=daily-question&envId=2025-06-12
pub struct Solution;

impl Solution {
    pub fn max_adjacent_distance(mut nums: Vec<i32>) -> i32 {
        nums.push(nums[0]);
        nums.windows(2)
            .map(|window| window[0].abs_diff(window[1]) as i32)
            .max()
            .unwrap_or(0)
    }
}

fn main() {
    println!("Hello, world!");
}
