// https://leetcode.com/problems/sum-of-all-subset-xor-totals/description/
pub struct Solution;

impl Solution {
    pub fn subset_xor_sum(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut bitmask = 0;
        let max = 1 << n;
        let mut sum = 0;

        while bitmask < max {
            let xor = nums.iter().rev().enumerate().fold(0, |xor, (pos, num)| {
                if (bitmask >> pos) & 1 == 1 {
                    xor ^ num
                } else {
                    xor
                }
            });
            sum += xor;

            bitmask += 1;
        }

        sum
    }
}

fn main() {
    println!("Hello, world!");
}
