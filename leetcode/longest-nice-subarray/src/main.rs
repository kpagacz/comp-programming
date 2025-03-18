// https://leetcode.com/problems/longest-nice-subarray/description/?envType=daily-question&envId=2025-03-18
pub struct Solution;

impl Solution {
    pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
        let mut longest = 0;
        let mut ones: [i32; 32] = [0; 32];

        let mut left = 0usize;
        for (pos, &num) in nums.iter().enumerate() {
            (0..32).for_each(|bit_shift| {
                if (num >> bit_shift) & 1 == 1 {
                    ones[bit_shift] += 1;
                }
            });

            while left < pos && !ones.iter().all(|pos| pos <= &1) {
                (0..32).for_each(|bit_shift| {
                    if (nums[left] >> bit_shift) & 1 == 1 {
                        ones[bit_shift] -= 1;
                    }
                });
                left += 1;
            }

            longest = longest.max(pos - left + 1);
        }

        longest as _
    }

    pub fn longest_nice_subarray_nicer(nums: Vec<i32>) -> i32 {
        let mut longest = 1;
        let mut bitmask = nums[0];
        let mut left = 0usize;

        for (pos, &num) in nums.iter().enumerate().skip(1) {
            while bitmask & num != 0 {
                bitmask ^= nums[left];
                left += 1;
            }

            bitmask |= num;
            longest = longest.max(pos - left + 1);
        }
        longest as _
    }
}

fn main() {
    println!("Hello, world!");
}
