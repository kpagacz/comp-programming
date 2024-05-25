// https://leetcode.com/problems/word-break-ii/description/
pub struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> Vec<String> {
        use std::collections::HashSet;
        fn backtrack(
            s: &str,
            words_so_far: &mut Vec<String>,
            dict: &HashSet<String>,
            answer: &mut Vec<String>,
        ) {
            if s.is_empty() {
                answer.push(words_so_far.join(" ").to_string());
                return;
            }

            for i in 1..=s.len() {
                if dict.contains(&s[..i]) {
                    words_so_far.push(s[0..i].to_string());
                    backtrack(&s[i..], words_so_far, dict, answer);
                    words_so_far.pop();
                }
            }
        }

        let mut words_so_far = vec![];
        let dict = HashSet::<String>::from_iter(word_dict);
        let mut answer = vec![];
        backtrack(&s[..], &mut words_so_far, &dict, &mut answer);
        answer
    }
}

fn main() {
    let test_cases = [("catsanddog", vec!["cat", "cats", "and", "sand", "dog"])];

    for (s, word_dict) in test_cases {
        println!(
            "{:#?}",
            Solution::word_break(
                s.to_string(),
                word_dict.into_iter().map(str::to_string).collect()
            )
        );
    }
}
