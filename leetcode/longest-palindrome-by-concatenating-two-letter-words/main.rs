// https://leetcode.com/problems/longest-palindrome-by-concatenating-two-letter-words/?envType=daily-question&envId=2025-05-25
pub struct Solution;

impl Solution {
    pub fn longest_palindrome(words: Vec<String>) -> i32 {
        use std::collections::HashMap;

        let mut map = HashMap::new();
        let mut longest = 0;
        for word in words {
            let reversed = word.clone().chars().rev().collect::<String>();
            if map.get(&reversed).is_some_and(|count| *count > 0 ) {
                longest += 4;
                map.entry(reversed).and_modify(|entry| *entry -= 1).or_insert(0);
            } else {
                map.entry(word).and_modify(|entry| *entry += 1).or_insert(1);
            }
        }
        longest + if map.into_iter().any(|(key, value)| &key[..1] == &key[1..] && value % 2 == 1) { 2 } else { 0 }
        }
}

fn main() {
    println!("Hello world");
}