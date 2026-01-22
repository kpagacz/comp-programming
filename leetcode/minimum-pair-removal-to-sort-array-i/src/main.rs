// https://leetcode.com/problems/minimum-pair-removal-to-sort-array-i/description/?envType=daily-question&envId=2026-01-22
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn minimum_pair_removal(mut nums: Vec<i32>) -> i32 {
        let mut operations = 0;
        fn is_non_decreasing(arr: &[i32]) -> bool {
            arr.windows(2).all(|window| window[1] >= window[0])
        }
        while !is_non_decreasing(&nums) {
            let mut min_sum = (i32::MAX, 0);
            nums.windows(2).enumerate().for_each(|(pos, window)| {
                if min_sum.0 > window[0] + window[1] {
                    min_sum.0 = window[0] + window[1];
                    min_sum.1 = pos;
                }
            });
            let mut new_nums = vec![];
            let mut i = 0;
            while i < nums.len() {
                if i == min_sum.1 {
                    new_nums.push(nums[i] + nums[i + 1]);
                    i += 2;
                } else {
                    new_nums.push(nums[i]);
                    i += 1;
                }
            }
            nums = new_nums;
            operations += 1;
        }
        operations
    }
}

fn main() {
    let test_cases = [(vec![5, 2, 3, 1], 2)];
    for (nums, exp) in test_cases {
        println!("{} exp: {exp}", Solution::minimum_pair_removal(nums));
    }
}
