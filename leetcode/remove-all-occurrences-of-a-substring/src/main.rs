// https://leetcode.com/problems/remove-all-occurrences-of-a-substring/description/
pub struct Solution;

impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let mut stack = vec![];
        let part = part.as_bytes();

        for &b in s.as_bytes() {
            stack.push(b);

            if stack.ends_with(part) {
                for _ in 0..part.len() {
                    stack.pop();
                }
            }
        }

        String::from_utf8(stack).unwrap()
    }
}

fn main() {
    println!("Hello, world!");
}
