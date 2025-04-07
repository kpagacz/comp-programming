// https://leetcode.com/problems/partition-equal-subset-sum/description/?envType=daily-question&envId=2025-04-07
pub struct Solution;

impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let max = n * 100usize;
        let mut dp = vec![vec![false; max + 1]; n + 1];

        let sum = nums.iter().sum::<i32>();
        if sum % 2 == 1 {
            return false;
        }
        let target = sum / 2;

        dp[0][0] = true;
        for i in 0..n {
            for num in 0..=(max - nums[i] as usize) {
                dp[i + 1][num + nums[i] as usize] |= dp[i][num];
                dp[i + 1][num] |= dp[i][num];
            }
        }

        dp[n][target as usize]
    }
}

fn main() {
    println!("Hello, world!");
}
