// https://leetcode.com/problems/monotonic-array/description/
pub struct Solution {}

impl Solution {
    pub fn is_monotonic(nums: Vec<i32>) -> bool {
        nums.windows(2).all(|el| el[0] <= el[1]) || nums.windows(2).all(|el| el[0] >= el[1])
    }
}
fn main() {
    println!("Hello, world!");
}
