// https://leetcode.com/problems/reverse-words-in-a-string-iii/description/
pub struct Solution {}
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut bytes = s.as_bytes().to_owned();
        let mut it = 0;
        while let Some(word_start) =
            s[it..].find(|c: char| c.is_alphabetic() || c.is_ascii_punctuation())
        {
            let offset_start = word_start + it;
            match s[offset_start..].find(char::is_whitespace) {
                Some(after_word) => {
                    bytes[offset_start..(offset_start + after_word)].reverse();
                    it = offset_start + after_word;
                }
                None => {
                    bytes[offset_start..].reverse();
                    break;
                }
            }
        }

        String::from_utf8(bytes).unwrap()
    }

    // The constraint is that the words are separated by a single space...
    pub fn reverse_words2(s: String) -> String {
        let split: Vec<&str> = s.split_whitespace().collect();
        split
            .iter()
            .map(|fragment| fragment.chars().rev().collect::<String>())
            .collect::<Vec<String>>()
            .join(" ")
    }
}
fn main() {
    let test_cases = vec![
        "Let's take LeetCode contest",
        "God Ding",
        "     something  asd7hkljh&asld",
    ];
    for test in test_cases {
        println!("{}", Solution::reverse_words2(test.to_owned()));
    }
}
