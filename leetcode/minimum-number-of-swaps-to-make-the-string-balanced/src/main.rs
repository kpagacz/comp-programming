// https://leetcode.com/problems/minimum-number-of-swaps-to-make-the-string-balanced/description/
pub struct Solution;

impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut stack = Vec::default();

        for c in s.chars() {
            match c {
                '[' => stack.push(c),
                ']' => {
                    if let Some(top) = stack.last() {
                        if top == &'[' {
                            stack.pop();
                        } else {
                            stack.push(c);
                        }
                    } else {
                        stack.push(c);
                    }
                }
                _ => unreachable!(),
            }
        }

        ((stack.len() + 3) / 4) as _
    }
}

fn main() {
    println!("Hello, world!");
}
