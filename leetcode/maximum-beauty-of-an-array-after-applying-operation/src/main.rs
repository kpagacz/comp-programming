// https://leetcode.com/problems/maximum-beauty-of-an-array-after-applying-operation/description/
pub struct Solution;

impl Solution {
    pub fn maximum_beauty(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();

        let mut max = 0;
        let mut left = 0;
        for i in 0..nums.len() {
            while left < i && nums[left] + 2 * k < nums[i] {
                left += 1;
            }

            if nums[left] + 2 * k >= nums[i] {
                max = max.max(i - left + 1);
            }
        }
        max as _
    }
}

fn main() {
    println!("Hello, world!");
}
