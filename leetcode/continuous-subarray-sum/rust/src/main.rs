// https://leetcode.com/problems/continuous-subarray-sum/description/
pub struct Solution;

impl Solution {
    pub fn check_subarray_sum(nums: Vec<i32>, k: i32) -> bool {
        let prefix = std::iter::once(0).chain(nums.into_iter().scan(0, |acc, num| {
            *acc += num;
            Some(*acc % k)
        }));
        use std::collections::HashSet;
        let mut seen = HashSet::new();
        let mut last = i32::MIN;
        for modulo in prefix {
            if seen.contains(&modulo) {
                return true;
            }
            seen.insert(last);
            last = modulo;
        }
        false
    }
}

fn main() {
    let test_cases = vec![(vec![23, 2, 6, 4, 7], 13), (vec![0], 1)];
    for (nums, k) in test_cases {
        println!("{}", Solution::check_subarray_sum(nums, k));
    }
}
