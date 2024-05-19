// https://leetcode.com/problems/find-the-maximum-sum-of-node-values/description/?envType=daily-question&envId=2024-05-19
pub struct Solution;

impl Solution {
    pub fn maximum_value_sum(nums: Vec<i32>, k: i32, edges: Vec<Vec<i32>>) -> i64 {
        let xored_diffs: Vec<_> = nums.iter().map(|&num| (num ^ k) - num).collect();

        let mut total = 0i64;
        let mut max_negative = i64::MIN;
        let mut min_positive = i64::MAX;
        let mut count = 0;

        // println!("{xored_diffs:#?}");
        for (pos, &xored_diff) in xored_diffs.iter().enumerate() {
            if xored_diff > 0 {
                count += 1;
                total += (nums[pos] ^ k) as i64;
                min_positive = min_positive.min(xored_diff as _);
            } else {
                total += nums[pos] as i64;
                max_negative = max_negative.max(xored_diff as _);
            }
        }

        // All xored
        if count == nums.len() && count % 2 == 1 {
            // println!("All xored");
            total -= min_positive;
        }

        // Not all xored and odd xored added, so need to remove one xored or add one negative
        // If addition of both is better than 0, then add
        if count < nums.len() && count % 2 == 1 {
            // println!("Add one negative");
            if max_negative + min_positive >= 0 {
                total += max_negative;
            } else {
                total -= min_positive;
            }
        }

        total
    }
}

fn main() {
    println!("Hello, world!");
}
