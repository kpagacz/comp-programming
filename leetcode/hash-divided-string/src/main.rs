// https://leetcode.com/problems/hash-divided-string/
pub struct Solution;

impl Solution {
    pub fn string_hash(s: String, k: i32) -> String {
        let s = s.as_bytes();
        s.chunks(k as usize)
            .map(|chunk| {
                let sum = chunk.iter().map(|&c| (c - b'a') as i32).sum::<i32>();
                let hashed_char = (sum % 26) as u8;
                (b'a' + hashed_char) as char
            })
            .collect::<String>()
    }
}

fn main() {
    println!("Hello, world!");
}
