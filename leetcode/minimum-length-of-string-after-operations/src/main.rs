// https://leetcode.com/problems/minimum-length-of-string-after-operations/description/
pub struct Solution;

impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        use std::collections::HashMap;

        let freq = s.chars().fold(HashMap::new(), |mut freq, c| {
            *freq.entry(c).or_insert(0) += 1;
            freq
        });

        freq.values()
            .map(|&freq| if freq % 2 == 1 { 1 } else { 2 })
            .sum::<i32>()
    }
}

fn main() {
    println!("Hello, world!");
}
