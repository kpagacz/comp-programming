// https://leetcode.com/problems/maximum-difference-between-increasing-elements/description/?envType=daily-question&envId=2025-06-16
pub struct Solution;

impl Solution {
    pub fn maximum_difference(nums: Vec<i32>) -> i32 {
        let mut stack = vec![];
        let mut max_diff = i32::MIN;
        for num in nums {
            if !stack.is_empty() && num > stack[0] {
                max_diff = max_diff.max(num - stack[0]);
            }
            while !stack.is_empty() && stack[stack.len() - 1] > num {
                stack.pop();
            }
            stack.push(num);
        }

        if max_diff == i32::MIN { -1 } else { max_diff }
    }
}

fn main() {
    println!("Hello, world!");
}
