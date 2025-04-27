// https://leetcode.com/problems/count-subarrays-of-length-three-with-a-condition/description/?envType=daily-question&envId=2025-04-27
pub struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>) -> i32 {
        nums.windows(3)
            .filter(|window| 2 * (window[0] + window[2]) == window[1])
            .count() as _
    }
}

fn main() {
    println!("Hello, world!");
}
