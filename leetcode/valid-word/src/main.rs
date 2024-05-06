// https://leetcode.com/contest/weekly-contest-396/problems/valid-word/
pub struct Solution;

impl Solution {
    pub fn is_valid(word: String) -> bool {
        let word = word.to_lowercase();
        let word = word.as_str();
        let is_vowel = |c: char| c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u';
        let is_consonant = |c: char| c.is_alphabetic() && !is_vowel(c);
        let is_not_alphanumeric = |c: char| !c.is_alphanumeric();

        word.len() >= 3
            && word.find(is_vowel).is_some()
            && word.find(is_consonant).is_some()
            && word.find(is_not_alphanumeric).is_none()
    }
}

fn main() {
    println!("Hello, world!");
}
