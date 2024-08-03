// https://leetcode.com/problems/make-two-arrays-equal-by-reversing-subarrays/description/
pub struct Solution;

impl Solution {
    pub fn can_be_equal(mut target: Vec<i32>, mut arr: Vec<i32>) -> bool {
        target.sort();
        arr.sort();
        arr == target
    }
}

fn main() {
    println!("Hello, world!");
}
