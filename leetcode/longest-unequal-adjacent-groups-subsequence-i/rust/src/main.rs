// https://leetcode.com/problems/longest-unequal-adjacent-groups-subsequence-i/
pub struct Solution {}

impl Solution {
    pub fn get_longest_subsequence(n: i32, words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let mut longest_ending_zero = vec![];
        let mut longest_ending_one = vec![];

        groups.iter().enumerate().for_each(|(id, &num)| {
            if num == 0 {
                if longest_ending_one.len() + 1 > longest_ending_zero.len() {
                    longest_ending_zero = longest_ending_one.clone();
                    longest_ending_zero.push(id);
                }
            } else {
                if longest_ending_zero.len() + 1 > longest_ending_one.len() {
                    longest_ending_one = longest_ending_zero.clone();
                    longest_ending_one.push(id);
                }
            }
        });

        if longest_ending_one.len() > longest_ending_zero.len() {
            longest_ending_one
                .iter()
                .map(|&id| words[id].clone())
                .collect()
        } else {
            longest_ending_zero
                .iter()
                .map(|&id| words[id].clone())
                .collect()
        }
    }
}

fn main() {
    println!("Hello, world!");
}
