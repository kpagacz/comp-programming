// https://leetcode.com/problems/check-if-two-string-arrays-are-equivalent/description/
pub struct Solution {}

impl Solution {
    pub fn array_strings_are_equal(word1: Vec<String>, word2: Vec<String>) -> bool {
        word1
            .iter()
            .map(|s| s.as_bytes())
            .flatten()
            .eq(word2.iter().map(|s| s.as_bytes()).flatten())
    }
}

fn main() {
    println!("Hello, world!");
}
