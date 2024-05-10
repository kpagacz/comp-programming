// https://leetcode.com/problems/distribute-candies/description/
pub struct Solution;

impl Solution {
    pub fn distribute_candies(candy_type: Vec<i32>) -> i32 {
        use std::collections::HashSet;
        let n = candy_type.len();
        (n / 2).min(HashSet::<i32>::from_iter(candy_type).len()) as _
    }
}

fn main() {
    println!("Hello, world!");
}
