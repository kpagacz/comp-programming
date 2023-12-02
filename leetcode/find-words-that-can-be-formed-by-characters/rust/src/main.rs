// https://leetcode.com/problems/find-words-that-can-be-formed-by-characters/description
pub struct Solution {}

impl Solution {
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        use std::collections::BTreeMap;
        let char_counts = chars.as_bytes().iter().fold(BTreeMap::new(), |mut map, c| {
            *map.entry(*c).or_insert(0) += 1;
            map
        });

        words
            .into_iter()
            .filter_map(|word| {
                let n = word.len();
                let word_map = word.as_bytes().iter().fold(BTreeMap::new(), |mut map, c| {
                    *map.entry(*c).or_insert(0) += 1;
                    map
                });
                for (c, count) in word_map {
                    if char_counts.get(&c).unwrap_or(&0) < &count {
                        return None;
                    }
                }
                Some(n as i32)
            })
            .sum()
    }
}
fn main() {
    println!("Hello, world!");
}
