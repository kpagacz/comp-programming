// https://leetcode.com/problems/shortest-subarray-with-or-at-least-k-i/description/
pub struct Solution;

impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut bits = [0; 32];
        let mut min_length = usize::MAX;

        fn num_from_bits(bits: &[i32]) -> i32 {
            let mut num = 0;
            for (pos, &freq) in bits.iter().rev().enumerate() {
                if freq > 0 {
                    num |= 1 << pos;
                }
            }
            num
        }

        fn modify_bits(bits: &mut [i32], num: i32, by: i32) {
            for i in 0..32 {
                if 1 & (num >> i) == 1 {
                    bits[bits.len() - 1 - i] += by;
                }
            }
        }

        let mut left = 0usize;
        for i in 0..nums.len() {
            modify_bits(&mut bits, nums[i], 1);

            while num_from_bits(&bits) >= k && left <= i {
                min_length = min_length.min(i - left + 1);
                modify_bits(&mut bits, nums[left], -1);
                left += 1;
            }
        }

        min_length as _
    }
}

fn main() {
    let test_cases = [(vec![2, 1, 8], 10), (vec![1, 2, 3], 2)];

    for (nums, k) in test_cases {
        println!("{}", Solution::minimum_subarray_length(nums, k));
    }
}
