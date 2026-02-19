// https://leetcode.com/problems/find-smallest-letter-greater-than-target/description/
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let first_greater = letters.partition_point(|&c| c <= target);
        if first_greater == letters.len() {
            letters[0]
        } else {
            letters[first_greater]
        }
    }
}

fn main() {
    println!("Hello, world!");
}
