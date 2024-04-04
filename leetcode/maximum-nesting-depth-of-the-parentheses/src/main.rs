// https://leetcode.com/problems/maximum-nesting-depth-of-the-parentheses/description/
pub struct Solution;

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let mut answer = 0;
        let mut depth = 0;

        for c in s.chars() {
            match c {
                ')' => depth -= 1,
                '(' => depth += 1,
                _ => {}
            }
            answer = answer.max(depth);
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
