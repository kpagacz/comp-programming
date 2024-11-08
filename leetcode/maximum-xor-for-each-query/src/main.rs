// https://leetcode.com/problems/maximum-xor-for-each-query/description/
pub struct Solution;

impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mut array_xor = nums.iter().fold(0, |acc, num| acc ^ *num);
        let mut answer = Vec::with_capacity(nums.len());

        let mask = (1 << maximum_bit) - 1;

        for i in 0..nums.len() {
            answer.push(array_xor ^ mask);
            array_xor ^= nums[nums.len() - 1 - i];
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
