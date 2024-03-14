// https://leetcode.com/problems/binary-subarrays-with-sum/description/?envType=daily-question&envId=2024-03-14
pub struct Solution;

impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        use std::collections::BTreeMap;
        let mut freqs = BTreeMap::new();
        freqs.insert(0, 1);

        let mut prefix = 0;
        let mut answer = 0;
        for num in nums {
            prefix += num;
            if let Some(freq) = freqs.get(&(prefix - goal)) {
                answer += freq;
            }
            *freqs.entry(prefix).or_insert(0) += 1;
        }

        answer
    }
}

fn main() {
    let tests = vec![(vec![1, 0, 1, 0, 1], 2)];

    for (nums, goal) in tests {
        println!("{}", Solution::num_subarrays_with_sum(nums, goal));
    }
}
