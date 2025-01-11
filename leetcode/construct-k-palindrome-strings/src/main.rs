// https://leetcode.com/problems/construct-k-palindrome-strings/description/
pub struct Solution;

impl Solution {
    pub fn can_construct(s: String, k: i32) -> bool {
        use std::collections::HashMap;

        let mut freq = s.as_bytes().iter().fold(HashMap::new(), |mut freq, &c| {
            *freq.entry(c).or_insert(0) += 1;
            freq
        });
        let odd = freq.values().filter(|&&v| v % 2 == 1).count() as i32;
        if s.len() < k as usize {
            return false;
        }
        if odd > k {
            return false;
        }
        true
    }
}

fn main() {
    println!("Hello, world!");
}
