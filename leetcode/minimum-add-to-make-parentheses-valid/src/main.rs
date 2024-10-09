// https://leetcode.com/problems/minimum-add-to-make-parentheses-valid/description/
pub struct Solution;

impl Solution {
    pub fn min_add_to_make_valid(s: String) -> i32 {
        let mut stack = vec![];

        for c in s.chars() {
            match c {
                ')' => {
                    if !stack.is_empty() && stack[stack.len() - 1] == '(' {
                        stack.pop();
                    } else {
                        stack.push(c);
                    }
                }
                '(' => stack.push(c),
                _ => unreachable!(),
            }
        }

        stack.len() as _
    }
}

fn main() {
    println!("Hello, world!");
}
