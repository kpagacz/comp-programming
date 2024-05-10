// https://leetcode.com/problems/longest-harmonious-subsequence/
pub struct Solution;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let freqs = nums.iter().fold(HashMap::new(), |mut map, num| {
            *map.entry(*num).or_insert(0) += 1;
            map
        });

        freqs
            .iter()
            .filter_map(|(key, value)| {
                let higher = freqs.get(&(*key + 1))?;
                Some(higher + value)
            })
            .max()
            .unwrap_or(0)
    }
}

fn main() {
    println!("Hello, world!");
}
