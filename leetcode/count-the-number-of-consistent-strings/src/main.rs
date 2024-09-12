// https://leetcode.com/problems/count-the-number-of-consistent-strings/description/
pub struct Solution;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let allowed = allowed
            .as_bytes()
            .iter()
            .fold(vec![false; 256], |mut acc, &b| {
                acc[b as usize] = true;
                acc
            });

        words
            .into_iter()
            .filter(|word| word.as_bytes().iter().all(|&b| allowed[b as usize]))
            .count() as _
    }
}

fn main() {
    println!("Hello, world!");
}
