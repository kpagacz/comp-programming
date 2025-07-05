// https://leetcode.com/problems/partition-array-to-minimize-xor/description/
pub struct Solution;

impl Solution {
    pub fn min_xor(nums: Vec<i32>, k: i32) -> i32 {
        // XOR of nums[l..r] = prefixes[l] ^ prefixes[r]
        // prefixes[0] = 0
        // prefixes[l] = nums[0] ^ ... ^ nums[l - 1]
        let mut prefixes = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            prefixes[i + 1] = nums[i] ^ prefixes[i];
        }

        // dp[i][k] = minimum "max XOR" when splitting first i elements into k parts
        let mut dp = vec![vec![i32::MAX; 1 + k as usize]; nums.len() + 1];
        for i in 1..=nums.len() {
            dp[i][1] = prefixes[i];
        }

        for i in 1..=nums.len() {
            for k in 2..=k as usize {
                let mut min = i32::MAX;
                for t in 0..i {
                    min = min.min(std::cmp::max(dp[t][k - 1], prefixes[t] ^ prefixes[i]));
                }
                dp[i][k] = min;
            }
        }

        dp[nums.len()][k as usize]
    }
}

fn main() {
    let test_cases = [(vec![1, 2, 3], 2, 1)];
    for (nums, k, exp) in test_cases {
        println!("{} exp {exp}", Solution::min_xor(nums, k));
    }
}
