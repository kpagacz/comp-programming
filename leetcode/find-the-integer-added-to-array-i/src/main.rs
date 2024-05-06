// https://leetcode.com/contest/weekly-contest-395/problems/find-the-integer-added-to-array-i/
pub struct Solution;

impl Solution {
    pub fn added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        (nums2.iter().sum::<i32>() - nums1.iter().sum::<i32>()) / nums1.len() as i32
    }
}

fn main() {
    println!("Hello, world!");
}
