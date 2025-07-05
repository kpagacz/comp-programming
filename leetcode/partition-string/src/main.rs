// https://leetcode.com/problems/partition-string/description/
pub struct Solution;

impl Solution {
    pub fn partition_string(s: String) -> Vec<String> {
        use std::collections::HashSet;

        let mut segments = HashSet::new();
        let mut answer = vec![];

        let mut start = 0;
        let mut it = 1;

        let s = s.as_str();

        while it <= s.len() {
            if !segments.contains(&s[start..it]) {
                segments.insert(&s[start..it]);
                answer.push(s[start..it].to_string());
                start = it;
            }

            it += 1;
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
