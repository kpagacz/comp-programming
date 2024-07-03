// https://leetcode.com/problems/minimum-difference-between-largest-and-smallest-value-in-three-moves/description/
pub struct Solution;

impl Solution {
    pub fn min_difference(mut nums: Vec<i32>) -> i32 {
        if nums.len() <= 4 {
            return 0;
        }

        nums.sort_unstable();

        let mut answer = i32::MAX;

        for i in 0..=3usize {
            let j = 3 - i;

            answer = answer.min(nums[nums.len() - 1 - j] - nums[i]);
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
