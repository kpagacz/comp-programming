// https://leetcode.com/problems/count-the-number-of-fair-pairs/description/
pub struct Solution;

impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        nums.sort_unstable();
        let mut answer = 0;

        for i in 0..nums.len() {
            let num = nums[i];
            let nums = &nums[i + 1..];
            let lower_than = lower - num;
            let pairs_with_sum_lower_than_lower =
                nums.partition_point(|&sorted| sorted < lower_than);

            let greater_than = upper - num;
            let pairs_with_sum_lower_equal_upper =
                nums.partition_point(|&sorted| sorted <= greater_than);

            answer +=
                pairs_with_sum_lower_equal_upper as i64 - pairs_with_sum_lower_than_lower as i64
        }

        answer
    }
}

fn main() {
    let test_cases = [
        (vec![0, 1, 7, 4, 4, 5], 3, 6),
        (vec![0, 0, 0, 0, 0, 0], 0, 0),
    ];
    for (nums, lower, upper) in test_cases {
        println!(
            "nums: {:?} {lower} {upper} ans:{}",
            &nums,
            Solution::count_fair_pairs(nums.clone(), lower, upper)
        );
    }
}
