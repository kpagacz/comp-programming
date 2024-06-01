// https://leetcode.com/problems/score-of-a-string/description/
pub struct Solution;

impl Solution {
    pub fn score_of_string(s: String) -> i32 {
        s.as_bytes()
            .windows(2)
            .map(|pair| pair[0].abs_diff(pair[1]) as i32)
            .sum::<i32>()
    }
}

fn main() {
    println!("Hello, world!");
}
