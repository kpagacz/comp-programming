//  https://leetcode.com/problems/crawler-log-folder/description/
pub struct Solution;

impl Solution {
    pub fn min_operations(logs: Vec<String>) -> i32 {
        let mut stack = Vec::new();

        for log in logs {
            match &log[..] {
                "../" => {
                    stack.pop();
                }
                "./" => {}
                _ => stack.push(0),
            }
        }

        stack.len() as _
    }
}

fn main() {
    println!("Hello, world!");
}
