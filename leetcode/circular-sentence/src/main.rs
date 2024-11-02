// https://leetcode.com/problems/circular-sentence/description/
pub struct Solution;

impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let mut words = sentence.split_whitespace().peekable();
        let first_word = words.next().unwrap();
        let first_char = first_word.chars().next().unwrap();
        let mut last_char = first_word.chars().last().unwrap();

        for word in words {
            let first_char = word.chars().next().unwrap();
            if last_char != first_char {
                return false;
            } else {
                last_char = word.chars().last().unwrap();
            }
        }

        first_char == last_char
    }
}

fn main() {
    println!("Hello, world!");
}
