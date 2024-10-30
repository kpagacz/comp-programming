// https://leetcode.com/problems/minimum-number-of-removals-to-make-mountain-array/description/
pub struct Solution;

impl Solution {
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let prefixes = Solution::min_removals_vec(&nums);
        let suffixes = Solution::min_removals_reverse_vec(&nums);
        // println!("{prefixes:?}");
        // println!("{suffixes:?}");

        (1..nums.len() - 1)
            .map(|mountain_top_index| {
                let prefix_removals = prefixes[mountain_top_index];
                let suffix_removals = suffixes[mountain_top_index];
                if prefix_removals != usize::MAX && suffix_removals != usize::MAX {
                    (prefix_removals + suffix_removals) as _
                } else {
                    i32::MAX
                }
            })
            .min()
            .unwrap()
    }

    fn min_removals_vec(arr: &[i32]) -> Vec<usize> {
        let mut dp = vec![usize::MAX; arr.len()]; // min removals when ending on this index
        dp[0] = 0;
        for i in 1..arr.len() - 1 {
            for j in (0..i).rev() {
                if arr[i] > arr[j] {
                    if dp[j] != usize::MAX {
                        dp[i] = dp[i].min(dp[j] + i - j - 1);
                    } else {
                        dp[i] = dp[i].min(i - 1)
                    }
                }
            }
        }
        dp
    }

    fn min_removals_reverse_vec(arr: &[i32]) -> Vec<usize> {
        let mut dp = vec![usize::MAX; arr.len()]; // min removals when ending on this index
        dp[arr.len() - 1] = 0;
        for i in (1..arr.len() - 1).rev() {
            for j in i + 1..arr.len() {
                if arr[i] > arr[j] {
                    if dp[j] != usize::MAX {
                        dp[i] = dp[i].min(dp[j] + j - i - 1);
                    } else {
                        dp[i] = dp[i].min(arr.len() - i - 2);
                    }
                }
            }
        }
        dp
    }
}

fn main() {
    let test_cases = [
        vec![1, 3, 1],
        vec![2, 1, 1, 5, 6, 2, 3, 1],
        vec![4, 3, 2, 1, 1, 2, 3, 1],
    ];
    for nums in test_cases {
        println!("{}", Solution::minimum_mountain_removals(nums));
    }
}
