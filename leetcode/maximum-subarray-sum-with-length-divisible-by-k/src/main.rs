// https://leetcode.com/problems/maximum-subarray-sum-with-length-divisible-by-k/description/?envType=daily-question&envId=2025-11-27
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let mut minimums = vec![BinaryHeap::new(); k as usize + 1];
        minimums[0].push(Reverse((0, 0)));

        let mut prefix = 0i64;
        let k = k as i64;
        for (pos, &num) in nums.iter().enumerate() {
            prefix += num as i64;
            minimums[(pos + 1) % k as usize].push(Reverse((prefix, pos + 1)));
        }

        let mut max_sum = i64::MIN;
        while let Some(&Reverse((saved_prefix, taken_nums))) =
            minimums[nums.len() % k as usize].peek()
        {
            if taken_nums < nums.len() {
                max_sum = prefix - saved_prefix;
                break;
            } else {
                minimums[nums.len() % k as usize].pop();
            }
        }
        (0..nums.len()).rev().for_each(|pos| {
            prefix -= nums[pos] as i64;
            while let Some(&Reverse((saved_prefix, taken_nums))) = minimums[pos % k as usize].peek()
            {
                if taken_nums < pos {
                    max_sum = max_sum.max(prefix - saved_prefix);
                    break;
                } else {
                    minimums[pos % k as usize].pop();
                }
            }
        });

        max_sum
    }
}
fn main() {
    let test_cases = [
        (vec![1, 2], 1, 3),
        (vec![-1, -2, -3, -4, -5], 4, -10),
        (vec![-5, 1, 2, -3, 4], 2, 4),
    ];

    for (nums, k, exp) in test_cases {
        println!("{}   exp: {exp}", Solution::max_subarray_sum(nums, k));
    }
}
