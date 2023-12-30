// https://leetcode.com/problems/redistribute-characters-to-make-all-strings-equal/
pub struct Solution;

impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        use std::collections::BTreeMap;
        let n = words.len();
        let chars = words.into_iter().fold(BTreeMap::new(), |map, word| {
            let map = word.chars().fold(map, |mut map, c| {
                *map.entry(c).or_insert(0) += 1;
                map
            });
            map
        });
        chars.values().all(|count| count % n == 0)
    }
}

fn main() {
    println!("Hello, world!");
}
