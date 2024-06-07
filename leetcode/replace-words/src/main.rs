// https://leetcode.com/problems/replace-words/description/?envType=daily-question&envId=2024-06-07
pub struct Solution;

impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        use std::collections::HashSet;
        let words = sentence.split(' ');
        let dictionary: HashSet<_> = HashSet::from_iter(dictionary);
        let replaced = words.map(|word| {
            for i in 1..word.len() {
                if dictionary.contains(&word[0..i]) {
                    return &word[0..i];
                }
            }
            word
        });

        replaced
            .fold(String::new(), |mut acc, word| {
                acc.reserve(word.len() + 1);
                acc.push_str(word);
                acc.push(' ');
                acc
            })
            .trim_end()
            .to_string()
    }
}

fn main() {
    let test_cases = [(
        vec!["Te".to_string(), "Some".to_string()],
        "Test test Something",
    )];
    for (dictionary, sentence) in test_cases {
        println!(
            "{}",
            Solution::replace_words(dictionary, sentence.to_owned())
        );
    }
}
