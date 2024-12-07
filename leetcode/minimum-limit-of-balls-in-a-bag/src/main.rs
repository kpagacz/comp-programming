// https://leetcode.com/problems/minimum-limit-of-balls-in-a-bag/description/
pub struct Solution;

impl Solution {
    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        let (mut left, mut right) = (1i64, *nums.iter().max().unwrap() as i64);
        let nums: Vec<_> = nums.into_iter().map(|num| num as i64).collect();

        while left != right {
            let mid = left + (right - left) / 2;

            let total_ops = nums
                .iter()
                .map(|&initial_bag| (initial_bag - 1) / mid)
                .sum::<i64>();

            if total_ops > max_operations as i64 {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left as _
    }
}

fn main() {
    let test_cases = [(vec![2, 4, 8, 2], 4)];
    for (nums, max_operations) in test_cases {
        println!("{}", Solution::minimum_size(nums, max_operations));
    }

    let nums = include_str!("../in");
    let nums: Vec<_> = nums
        .split(',')
        .filter_map(|num| num.parse::<i32>().ok())
        .collect();
    let max_ops = 908839574;
    println!("{}", Solution::minimum_size(nums, max_ops));
}
