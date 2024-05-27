// https://leetcode.com/problems/special-array-with-x-elements-greater-than-or-equal-x/description/
pub struct Solution;

impl Solution {
    pub fn special_array(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable_by_key(|x| -x);

        for i in 1..nums.len() {
            let higher = i as i32;
            if higher > nums[i] && higher <= nums[i - 1] {
                return higher;
            }
        }

        if *nums.first().unwrap() == 1 {
            return 1;
        }

        if (nums.len() as i32) <= *nums.last().unwrap() {
            return nums.len() as _;
        }

        -1
    }
}

fn main() {
    println!("Hello, world!");
}
