// https://leetcode.com/problems/reverse-string/
pub struct Solution;

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        let n = s.len();
        for i in 0..(n / 2) {
            s.swap(i, n - 1 - i);
        }
    }
}

fn main() {
    println!("Hello, world!");
}
