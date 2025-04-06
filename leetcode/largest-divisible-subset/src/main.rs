// https://leetcode.com/problems/largest-divisible-subset/description/?envType=daily-question&envId=2025-04-06
pub struct Solution;

impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut prev = vec![usize::MAX; n];
        let mut dp = vec![0; n];

        nums.sort_unstable();

        let mut max_id = usize::MAX;
        for i in 0..n {
            for j in 0..i {
                if nums[i] % nums[j] == 0 && dp[j] >= dp[i] {
                    dp[i] = dp[j] + 1;
                    prev[i] = j;
                }
            }

            if max_id == usize::MAX || dp[i] > dp[max_id] {
                max_id = i;
            }
        }

        let mut answer = Vec::default();
        while max_id != usize::MAX {
            answer.push(nums[max_id]);
            max_id = prev[max_id];
        }
        answer
    }
}

fn main() {
    println!("Hello, world!");
}
