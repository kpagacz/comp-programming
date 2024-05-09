// https://leetcode.com/problems/jump-game/description/
pub struct Solution;

impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let mut max_jump = 0usize;
        let mut it = 0usize;

        while it <= max_jump && it < nums.len() {
            max_jump = max_jump.max(it + nums[it] as usize);
            it += 1;
        }

        max_jump >= nums.len() - 1
    }
}

fn main() {
    println!("Hello, world!");
}
