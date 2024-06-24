// https://leetcode.com/problems/minimum-number-of-k-consecutive-bit-flips/description/
pub struct Solution;

impl Solution {
    pub fn min_k_bit_flips(mut nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut answer = 0;
        let mut current_flips = 0;

        for i in 0..nums.len() {
            if i >= k && nums[i - k] == 2 {
                current_flips -= 1;
            }

            if current_flips % 2 == nums[i] {
                if i + k > nums.len() {
                    return -1;
                }
                nums[i] = 2;
                answer += 1;
                current_flips += 1;
            }
        }

        answer
    }
}

fn main() {
    let test_cases = [
        (vec![1, 1, 0], 2),
        (vec![0, 1, 0], 1),
        (vec![0, 0, 0, 1, 0, 1, 1, 0], 3),
    ];
    for (nums, k) in test_cases {
        println!("{}", Solution::min_k_bit_flips(nums, k));
    }
}
