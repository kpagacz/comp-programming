// https://leetcode.com/problems/make-string-a-subsequence-using-cyclic-increments/description/
pub struct Solution;

impl Solution {
    pub fn can_make_subsequence(str1: String, str2: String) -> bool {
        if str2.len() > str1.len() {
            return false;
        }
        let mut it1 = 0usize;
        let mut it2 = 0usize;

        let str1 = str1.as_bytes();
        let str2 = str2.as_bytes();
        while it1 < str1.len() && it2 < str2.len() {
            if str1[it1] == str2[it2] || Solution::next_char(str1[it1]) == str2[it2] {
                it2 += 1;
            }
            it1 += 1;
        }

        it2 == str2.len()
    }

    fn next_char(c: u8) -> u8 {
        ((c - b'a' + 1) % 26) + b'a'
    }
}

fn main() {
    println!("Hello, world!");
}
