// https://leetcode.com/problems/check-if-a-word-occurs-as-a-prefix-of-any-word-in-a-sentence/description/?envType=daily-question&envId=2024-12-02
pub struct Solution;

impl Solution {
    pub fn is_prefix_of_word(sentence: String, search_word: String) -> i32 {
        sentence
            .split_whitespace()
            .position(|word| word.starts_with(&search_word))
            .map(|pos| 1 + pos as i32)
            .unwrap_or(-1) as _
    }
}

fn main() {
    println!("Hello, world!");
}
