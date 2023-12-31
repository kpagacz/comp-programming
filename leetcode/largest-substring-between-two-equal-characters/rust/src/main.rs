// https://leetcode.com/problems/largest-substring-between-two-equal-characters/
pub struct Solution;

impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        use std::collections::BTreeMap;
        let mut first_occurence = BTreeMap::new();
        let mut max_length = -1i32;
        s.chars().enumerate().for_each(|(pos, c)| {
            if let Some(first_pos) = first_occurence.get(&c) {
                max_length = max_length.max(pos as i32 - first_pos - 1);
            } else {
                first_occurence.insert(c, pos as i32);
            }
        });

        max_length
    }
}

fn main() {
    println!("Hello, world!");
}
