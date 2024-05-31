// https://leetcode.com/problems/single-number-iii/description/
pub struct Solution;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let xored = nums.iter().fold(0, |acc, num| acc ^ num);

        let differing_bit = xored ^ -xored;
        let (mut first, mut second) = (0, 0);

        nums.iter().for_each(|num| {
            if num ^ differing_bit == 0 {
                first ^= num;
            } else {
                second ^= num;
            }
        });

        vec![first, second]
    }
}

fn main() {
    println!("Hello, world!");
}
