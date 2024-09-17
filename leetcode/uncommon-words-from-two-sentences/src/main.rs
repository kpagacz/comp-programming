// https://leetcode.com/problems/uncommon-words-from-two-sentences/description/
pub struct Solution;

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        use std::collections::HashMap;

        let mut words = HashMap::new();
        for s in [s1, s2] {
            s.split_ascii_whitespace().for_each(|slice| {
                *words.entry(slice.to_string()).or_insert(0) += 1;
            });
        }

        words
            .into_iter()
            .filter_map(|(key, value)| if value == 1 { Some(key) } else { None })
            .collect()
    }
}

fn main() {
    println!("Hello, world!");
}
