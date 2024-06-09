// https://leetcode.com/problems/subarray-sums-divisible-by-k/description/
pub struct Solution;

impl Solution {
    pub fn subarrays_div_by_k(nums: Vec<i32>, k: i32) -> i32 {
        let prefix = std::iter::once(0).chain(nums.into_iter().scan(0, |acc, num| {
            *acc += num;
            Some(*acc % k)
        }));
        use std::collections::HashMap;
        let mut seen = HashMap::new();

        let mut answer = 0;
        for modulo in prefix {
            answer += seen.get(&modulo).unwrap_or(&0);
            answer += seen.get(&(modulo + k)).unwrap_or(&0);
            answer += seen.get(&(modulo - k)).unwrap_or(&0);
            *seen.entry(modulo).or_insert(0) += 1;
        }
        answer
    }
}

fn main() {
    let test_cases = [
        (vec![4, 5, 0, -2, -3, 1], 5),
        (vec![0], 5),
        (vec![], 5),
        (vec![-3, -7, -4, -3, -3, 20], 10),
        (vec![2, -7, 1], 6),
    ];
    for (nums, k) in test_cases {
        println!("{}", Solution::subarrays_div_by_k(nums, k));
    }
}
