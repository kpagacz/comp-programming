// https://leetcode.com/problems/split-a-string-into-the-max-number-of-unique-substrings/description
pub struct Solution;

impl Solution {
    pub fn max_unique_split(s: String) -> i32 {
        use std::collections::HashSet;

        fn backtrack<'a>(s: &'a str, seen: &mut HashSet<&'a str>) -> i32 {
            if s.is_empty() {
                return seen.len() as _;
            }

            let mut max_split = 0;
            for end in 1..=s.len() {
                if seen.contains(&s[..end]) {
                    continue;
                }
                seen.insert(&s[..end]);
                max_split = max_split.max(backtrack(&s[end..], seen));
                seen.remove(&s[..end]);
            }
            max_split
        }

        let s = s.as_str();
        backtrack(s, &mut HashSet::new())
    }
}

fn main() {
    println!("Hello, world!");
}
