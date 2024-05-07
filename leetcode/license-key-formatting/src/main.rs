// https://leetcode.com/problems/license-key-formatting/description/
pub struct Solution;

impl Solution {
    pub fn license_key_formatting(s: String, k: i32) -> String {
        let s = s.as_bytes();
        let k = k as usize;

        let mut answer = vec![];

        let mut added_chars = 0usize;
        for &b in s.iter().rev() {
            if b != b'-' {
                answer.push(b.to_ascii_uppercase());
                added_chars += 1;
                if added_chars % k == 0 {
                    answer.push(b'-');
                    added_chars = 0;
                }
            }
        }
        if let Some(c) = answer.last() {
            if c == &b'-' {
                answer.pop();
            }
        }

        String::from_utf8(answer.into_iter().rev().collect()).unwrap()
    }
}

fn main() {
    println!("Hello, world!");
}
