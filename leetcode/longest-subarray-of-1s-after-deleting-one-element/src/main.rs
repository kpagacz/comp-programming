// https://leetcode.com/problems/longest-subarray-of-1s-after-deleting-one-element/description/?envType=daily-question&envId=2025-08-24
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut answer = 0;

        let mut left = 0;
        let mut zeros = 0;

        for right in 0..nums.len() {
            if nums[right] == 0 {
                zeros += 1;
            }

            while zeros > 1 && left < right {
                if nums[left] == 0 {
                    zeros -= 1;
                }
                left += 1;
            }

            answer = answer.max(right - left);
        }

        answer as _
    }
}

fn main() {
    let test_cases = [
        vec![1, 1, 0, 1],
        vec![0, 1, 1, 1, 0, 1, 1, 0, 1],
        vec![1, 1, 1],
        vec![0, 0, 0, 0],
        vec![0, 0, 1, 1],
    ];

    for nums in test_cases {
        println!("{}", Solution::longest_subarray(nums));
    }
}
