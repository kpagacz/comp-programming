// https://leetcode.com/problems/find-common-characters/description/?envType=daily-question&envId=2024-06-05
pub struct Solution;

impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;
        let mut common = HashMap::new();
        for c in 'a'..='z' {
            common.insert(c, usize::MAX);
        }

        for word in words {
            let freq = word.chars().fold(HashMap::new(), |mut map, c| {
                *map.entry(c).or_insert(0) += 1;
                map
            });

            for c in 'a'..='z' {
                let entry = common.entry(c).or_insert(0);
                *entry = (*entry).min(*freq.get(&c).unwrap_or(&0));
            }
        }

        let mut answer = vec![];
        for c in 'a'..='z' {
            for _ in 0..*common.get(&c).unwrap_or(&0) {
                answer.push(c.to_string());
            }
        }
        answer
    }
}

fn main() {
    println!("Hello, world!");
}
