// https://leetcode.com/problems/clear-digits/description/
pub struct Solution;

impl Solution {
    pub fn clear_digits(s: String) -> String {
        let mut stack = vec![];
        for c in s.chars() {
            if c.is_ascii_digit() {
                stack.pop();
            } else {
                stack.push(c);
            }
        }
        stack.into_iter().collect()
    }
}

fn main() {
    println!("Hello, world!");
}
