// https://leetcode.com/problems/special-array-i/description/
pub struct Solution;

impl Solution {
    pub fn is_array_special(nums: Vec<i32>) -> bool {
        nums.windows(2)
            .all(|window| (window[0] ^ window[1]) & 1 == 1)
    }
}

fn main() {
    println!("Hello, world!");
}
