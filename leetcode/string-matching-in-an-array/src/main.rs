// https://leetcode.com/problems/string-matching-in-an-array/description/
pub struct Solution;

impl Solution {
    pub fn string_matching(words: Vec<String>) -> Vec<String> {
        let mut ans = vec![];
        for i in 0..words.len() {
            for j in 0..words.len() {
                if i == j {
                    continue;
                }
                if words[i].contains(&words[j]) {
                    ans.push(words[j].clone());
                }
            }
        }
        ans.sort();
        ans.dedup();
        ans
    }
}

fn main() {
    println!("Hello, world!");
}
