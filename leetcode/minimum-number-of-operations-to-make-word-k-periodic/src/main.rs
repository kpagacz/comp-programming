// https://leetcode.com/contest/weekly-contest-396/problems/minimum-number-of-operations-to-make-word-k-periodic/
pub struct Solution;

impl Solution {
    pub fn minimum_operations_to_make_k_periodic(word: String, k: i32) -> i32 {
        let word = word.as_str();

        use std::collections::HashMap;
        let mut i = 0;
        let k = k as usize;
        let mut map = HashMap::new();
        while i < word.len() {
            *map.entry(&word[i..i + k]).or_default() += 1;
            i += k;
        }

        map.values().sum::<i32>() - *map.values().max().unwrap()
    }
}

fn main() {
    println!("Hello, world!");
}
