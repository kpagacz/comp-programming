// https://leetcode.com/problems/number-of-segments-in-a-string/description/
pub struct Solution;

impl Solution {
    pub fn count_segments(s: String) -> i32 {
        let s = s.as_str();
        s.split_whitespace().count() as _
    }
}

fn main() {
    println!("Hello, world!");
}
