// https://leetcode.com/problems/neighboring-bitwise-xor/description/
pub struct Solution;

impl Solution {
    pub fn does_valid_array_exist(derived: Vec<i32>) -> bool {
        derived.into_iter().fold(0, |acc, num| acc ^ num) == 0
    }
}

fn main() {
    println!("Hello, world!");
}
