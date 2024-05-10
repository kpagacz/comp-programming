// https://leetcode.com/problems/array-partition/description/
pub struct Solution;

impl Solution {
    pub fn array_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        nums.chunks(2)
            .map(|chunk| chunk[0].min(chunk[1]))
            .sum::<i32>()
    }
}

fn main() {
    println!("Hello, world!");
}
