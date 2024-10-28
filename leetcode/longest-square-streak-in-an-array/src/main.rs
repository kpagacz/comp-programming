// https://leetcode.com/problems/longest-square-streak-in-an-array/description/
pub struct Solution;

impl Solution {
    pub fn longest_square_streak(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut mem = vec![false; 100_001];
        let mut dp = vec![1; 100_001];

        for i in 0..nums.len() {
            mem[nums[i] as usize] = true;
            let square_root = (nums[i] as f64).sqrt().round() as i32;
            if square_root * square_root == nums[i] && mem[square_root as usize] {
                dp[nums[i] as usize] = dp[square_root as usize] + 1;
            }
        }

        let max = dp.into_iter().max().unwrap();
        if max == 1 {
            -1
        } else {
            max
        }
    }
}

fn main() {
    println!("Hello, world!");
}
