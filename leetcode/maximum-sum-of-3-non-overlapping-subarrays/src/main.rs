// https://leetcode.com/problems/maximum-sum-of-3-non-overlapping-subarrays/description/
pub struct Solution;

impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut dp = vec![vec![(0, 0); 4]; nums.len() + 1];
        let k = k as usize;

        for says in 1..=3 {
            let mut current_sum = nums.iter().take(k).sum::<i32>();
            for i in k..dp.len() {
                if dp[i - 1][says].0 >= dp[i - k][says - 1].0 + current_sum {
                    dp[i][says].1 = dp[i - 1][says].1;
                } else {
                    dp[i][says].1 = i - k;
                }
                dp[i][says].0 = dp[i - 1][says].0.max(dp[i - k][says - 1].0 + current_sum);
                if i < nums.len() {
                    current_sum += nums[i];
                    current_sum -= nums[i - k];
                }
            }
        }

        let mut ans = vec![];
        let mut it = dp[nums.len()][3].1;
        for i in 1..4 {
            ans.push(it as i32);
            it = dp[it][3 - i].1;
        }
        ans.into_iter().rev().collect()
    }
}

fn main() {
    let test_cases = [
        (vec![1, 2, 1, 2, 6, 7, 5, 1], 2),
        (vec![1, 2, 1, 2, 1, 2, 1, 2, 1], 2),
    ];

    for (nums, k) in test_cases {
        println!("{:?}", Solution::max_sum_of_three_subarrays(nums, k));
    }
}
