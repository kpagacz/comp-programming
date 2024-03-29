// https://leetcode.com/problems/count-subarrays-where-max-element-appears-at-least-k-times/description/
pub struct Solution;

impl Solution {
    pub fn count_subarrays(nums: Vec<i32>, k: i32) -> i64 {
        let mut max_count = 0;
        let mut answer = 0;
        let mut left = 0;

        let max = *nums.iter().max().unwrap();

        for (i, &num) in nums.iter().enumerate() {
            if num == max {
                max_count += 1;
            }
            while left <= i && max_count >= k {
                answer += nums.len() - i;
                if nums[left] == max {
                    max_count -= 1;
                }
                left += 1;
            }
        }

        answer as _
    }
}

fn main() {
    let tests = [(vec![3, 3, 3, 3], 2)];
    for (nums, k) in tests {
        println!("{}", Solution::count_subarrays(nums, k));
    }
}
