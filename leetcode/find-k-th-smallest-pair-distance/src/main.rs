// https://leetcode.com/problems/find-k-th-smallest-pair-distance/description/
pub struct Solution;

impl Solution {
    pub fn smallest_distance_pair(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let k = k as usize;
        let (mut left, mut right) = (0, nums[nums.len() - 1] - nums[0]);

        while left < right {
            let mid = left + (right - left) / 2;

            let mut pairs = 0;
            let mut begin = 0;

            for i in 0..nums.len() {
                while nums[i] - nums[begin] > mid {
                    begin += 1;
                }

                pairs += i - begin;
            }

            if pairs >= k {
                right = mid;
            } else {
                left = mid + 1;
            }
        }

        left
    }
}

fn main() {
    println!("Hello, world!");
}
