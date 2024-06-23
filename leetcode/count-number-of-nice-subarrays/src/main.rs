//  https://leetcode.com/problems/count-number-of-nice-subarrays/description/
pub struct Solution;

impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        let mut odd_map = HashMap::new();
        odd_map.insert(0, 1);

        let mut answer = 0;
        let mut current = 0;
        for num in nums {
            if num % 2 == 1 {
                current += 1;
            }

            let additional_odds = current - k;
            if additional_odds >= 0 {
                answer += odd_map.get(&additional_odds).unwrap_or(&0);
            }
            *odd_map.entry(current).or_insert(0) += 1;
            // println!("{num} {answer} {current} {odd_map:#?}");
        }
        answer
    }
}

fn main() {
    let test_cases = [(vec![2, 2, 2, 1, 2, 2, 1, 2, 2, 2], 2)];
    for (nums, k) in test_cases {
        println!("{}", Solution::number_of_subarrays(nums, k));
    }
}
