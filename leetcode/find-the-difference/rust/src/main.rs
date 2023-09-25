// https://leetcode.com/problems/find-the-difference/description
pub struct Solution {}

impl Solution {
    pub fn find_the_difference(s: String, t: String) -> char {
        let mut first = vec![0; 30];

        let a = 'a' as usize;
        for c in t.chars() {
            first[c as usize - a] += 1;
        }
        for c in s.chars() {
            first[c as usize - a] -= 1;
        }

        for i in 0..first.len() {
            if first[i] == 1 {
                return ((a + i) as u8) as char;
            }
        }

        unreachable!()
    }

    // apparently it's just bytes, but I didn't come up with it on my own
    // I learned today that I can just tread chars as bytes in rust.
    // I didn't know what to do with chars in a String, but it seems that
    // treating them as bytes is a decent way to go if you know all the chars
    // fit into u8
    pub fn find_the_difference_fast(s: String, t: String) -> char {
        let first_sum: u32 = s.as_bytes().iter().map(|&byte| byte as u32).sum();
        let second_sum: u32 = t.as_bytes().iter().map(|&byte| byte as u32).sum();
        (second_sum - first_sum) as u8 as char
    }
}

fn main() {
    println!("Hello, world!");
}
