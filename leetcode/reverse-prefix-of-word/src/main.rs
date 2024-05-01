// https://leetcode.com/problems/reverse-prefix-of-word/description/
struct Solution;

impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        match word.find(ch) {
            None => word,
            Some(end) => {
                let word = word.as_str();
                word[..=end].chars().rev().collect::<String>() + &word[end + 1..]
            }
        }
    }
}

fn main() {
    let test_cases = [("abcdefd", 'd'), ("abcd", 'd')];
    for (word, ch) in test_cases {
        println!("{}", Solution::reverse_prefix(word.to_string(), ch));
    }
}
