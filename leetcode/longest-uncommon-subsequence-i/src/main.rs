// https://leetcode.com/problems/longest-uncommon-subsequence-i/description/
pub struct Solution;

impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        if a == b {
            -1
        } else {
            a.len().max(b.len()) as _
        }
    }
}

fn main() {
    println!("Hello, world!");
}
