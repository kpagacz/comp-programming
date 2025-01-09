// https://leetcode.com/problems/counting-words-with-a-given-prefix/description/
pub struct Solution;

impl Solution {
    pub fn prefix_count(words: Vec<String>, pref: String) -> i32 {
        words.into_iter().filter(|s| s.starts_with(&pref)).count() as _
    }
}

fn main() {
    println!("Hello, world!");
}
