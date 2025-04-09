// https://leetcode.com/problems/minimum-operations-to-make-array-values-equal-to-k/description/?envType=daily-question&envId=2025-04-09
pub struct Solution;

impl Solution {
    pub fn min_operations(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        nums.dedup();
        if k > nums[0] {
            return -1;
        }
        let n = nums.len();
        let pp = nums.partition_point(|num| num <= &k);
        (n - pp) as _
    }
}

fn main() {
    println!("Hello, world!");
}
