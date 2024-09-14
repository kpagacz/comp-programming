// https://leetcode.com/problems/longest-subarray-with-maximum-bitwise-and/description/
pub struct Solution;

impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let max_el = *nums.iter().max().unwrap();

        let mut answer = 0;
        let mut current = 0;
        for &num in &nums {
            if num == max_el {
                current += 1;
            } else {
                answer = answer.max(current);
                current = 0;
            }
        }
        answer = answer.max(current);

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
