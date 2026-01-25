// https://leetcode.com/problems/minimum-difference-between-highest-and-lowest-of-k-scores/description/?envType=daily-question&envId=2026-01-25
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let k = k as usize;
        let mut min = i32::MAX;
        for i in k - 1..nums.len() {
            min = min.min(nums[i] - nums[i + 1 - k]);
        }
        min
    }
}
fn main() {
    println!("Hello, world!");
}
