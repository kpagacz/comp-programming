// https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/description/
pub struct Solution {}

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let start = nums.partition_point(|el| el < &target);
        let end = nums.partition_point(|el| el <= &target);

        if start == end || nums[start] != target {
            vec![-1, -1]
        } else {
            vec![start as _, end as _]
        }
    }
}

fn main() {
    println!("Hello, world!");
}
