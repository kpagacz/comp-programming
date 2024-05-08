// https://leetcode.com/problems/keyboard-row/description
pub struct Solution;

impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let first_row = "qwertyuiop".as_bytes();
        let second_row = "asdfghjkl".as_bytes();
        let third_row = "zxcvbnm".as_bytes();

        let is_written_by_one_row = |word: &String| {
            let word = word.as_bytes();
            word.iter()
                .all(|c| first_row.contains(&c.to_ascii_lowercase()))
                || word
                    .iter()
                    .all(|c| second_row.contains(&c.to_ascii_lowercase()))
                || word
                    .iter()
                    .all(|c| third_row.contains(&c.to_ascii_lowercase()))
        };

        words.into_iter().filter(is_written_by_one_row).collect()
    }
}

fn main() {
    println!("Hello, world!");
}
