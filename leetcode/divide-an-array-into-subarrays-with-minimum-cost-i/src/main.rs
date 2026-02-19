// https://leetcode.com/problems/divide-an-array-into-subarrays-with-minimum-cost-i/description/?envType=daily-question&envId=2026-02-01
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn minimum_cost(mut nums: Vec<i32>) -> i32 {
        let first = nums[0];
        let _ = &nums[1..].sort_unstable();
        first + nums[1..].iter().take(2).sum::<i32>()
    }
}

fn main() {
    println!("Hello, world!");
}
