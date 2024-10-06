// https://leetcode.com/problems/sentence-similarity-iii/description/
pub struct Solution;

impl Solution {
    pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
        let s1: Vec<_> = sentence1.split_whitespace().collect();
        let s2: Vec<_> = sentence2.split_whitespace().collect();

        let mut prefix = 0;
        while prefix < s1.len() && prefix < s2.len() && s1[prefix] == s2[prefix] {
            prefix += 1;
        }

        let mut suffix = 0;
        let n1 = s1.len();
        let n2 = s2.len();
        while suffix < s1.len() && suffix < s2.len() && s1[n1 - 1 - suffix] == s2[n2 - 1 - suffix] {
            suffix += 1;
        }

        (prefix + suffix >= n1) || (prefix + suffix >= n2)
    }

    pub fn are_sentences_similar_it(sentence1: String, sentence2: String) -> bool {
        let mut s1 = sentence1.split_whitespace().peekable();
        let mut s2 = sentence2.split_whitespace().peekable();

        loop {
            match (s1.peek(), s2.peek()) {
                (Some(w1), Some(w2)) => {
                    if w1 == w2 {
                        s1.next();
                        s2.next();
                    } else {
                        break;
                    }
                }
                (None, Some(_)) | (Some(_), None) | (None, None) => return true,
            }
        }

        loop {
            match (s1.next_back(), s2.next_back()) {
                (Some(w1), Some(w2)) => {
                    if w1 != w2 {
                        return false;
                    }
                }
                (None, Some(_)) | (Some(_), None) | (None, None) => return true,
            }
        }
    }
}

fn main() {
    let test_cases = [
        ("of", "A lot of words"),
        ("My name is Haley", "My Haley"),
        ("Eating right now", "Eating"),
        ("A", "a A b A"),
    ];
    let res = [false, true, true, true];
    for ((s1, s2), expected) in test_cases.into_iter().zip(res) {
        println!(
            "{} expected {expected}",
            Solution::are_sentences_similar_it(s1.to_string(), s2.to_string())
        );
    }
}
