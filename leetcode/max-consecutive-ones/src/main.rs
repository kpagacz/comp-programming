// https://leetcode.com/problems/max-consecutive-ones/description/
pub struct Solution;

impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max = i32::MIN;
        let mut curr = 0;

        for num in nums {
            if num == 1 {
                curr += 1;
            } else {
                max = max.max(curr);
                curr = 0;
            }
        }
        max = max.max(curr);

        max
    }
}

fn main() {
    println!("Hello, world!");
}
