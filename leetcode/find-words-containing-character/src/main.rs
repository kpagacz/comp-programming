// https://leetcode.com/problems/find-words-containing-character/description/?envType=daily-question&envId=2025-05-24
pub struct Solution;

impl Solution {
    pub fn find_words_containing(words: Vec<String>, x: char) -> Vec<i32> {
        words
            .into_iter()
            .enumerate()
            .filter_map(|(i, word)| {
                if word.contains(x) {
                    Some(i as i32)
                } else {
                    None
                }
            })
            .collect()
    }
}

fn main() {
    println!("Hello, world!");
}
