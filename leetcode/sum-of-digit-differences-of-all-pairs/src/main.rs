// https://leetcode.com/problems/sum-of-digit-differences-of-all-pairs/description/
pub struct Solution;

impl Solution {
    pub fn sum_digit_differences(nums: Vec<i32>) -> i64 {
        let mut answer = 0;
        use std::collections::HashMap;

        let digit_num = nums[0].to_string().len();
        let mut freqs_at_pos = vec![HashMap::<u8, i64>::new(); digit_num];

        for (pos, num) in nums.into_iter().enumerate() {
            let num = num.to_string();
            let num = num.as_bytes();
            let nums_so_far = pos as i64;

            for (digit_pos, digit) in num.iter().enumerate() {
                answer += nums_so_far - freqs_at_pos[digit_pos].get(digit).unwrap_or(&0);
                *freqs_at_pos[digit_pos].entry(*digit).or_insert(0) += 1;
            }
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
