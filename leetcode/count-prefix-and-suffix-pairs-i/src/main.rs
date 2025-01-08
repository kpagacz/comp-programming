// https://leetcode.com/problems/count-prefix-and-suffix-pairs-i/description/
pub struct Solution;

impl Solution {
    pub fn count_prefix_suffix_pairs(words: Vec<String>) -> i32 {
        let mut answer = 0;
        for i in 0..words.len() {
            for j in i + 1..words.len() {
                if words[j].starts_with(&words[i]) && words[j].ends_with(&words[i]) {
                    answer += 1;
                }
            }
        }
        answer
    }
}

fn main() {
    println!("Hello, world!");
}
