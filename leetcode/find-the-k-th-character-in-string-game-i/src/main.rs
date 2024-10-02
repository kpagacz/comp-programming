// https://leetcode.com/problems/find-the-k-th-character-in-string-game-i/description/
pub struct Solution;

impl Solution {
    pub fn kth_character(k: i32) -> char {
        (b'a' + ((k - 1).count_ones() % 26) as u8) as char
    }
}

fn main() {
    println!("Hello, world!");
}
