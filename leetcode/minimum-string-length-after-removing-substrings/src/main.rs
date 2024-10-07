// https://leetcode.com/problems/minimum-string-length-after-removing-substrings/description/
pub struct Solution;

impl Solution {
    pub fn min_length(s: String) -> i32 {
        let mut stack = Vec::default();

        for c in s.chars() {
            stack.push(c);

            while stack.len() >= 2 {
                match stack[stack.len() - 2..] {
                    ['A', 'B'] | ['C', 'D'] => {
                        stack.pop();
                        stack.pop();
                    }
                    _ => break,
                }
            }
        }

        stack.len() as _
    }
}

fn main() {
    println!("Hello, world!");
}
