// https://leetcode.com/problems/find-longest-special-substring-that-occurs-thrice-i/description/
pub struct Solution;

impl Solution {
    pub fn maximum_length(s: String) -> i32 {
        use std::collections::HashMap;

        let s = s.as_bytes();
        let mut substrings: HashMap<&[u8], i32> = HashMap::default();

        for i in 0..s.len() {
            let mut k = 0;
            while i + k < s.len() && s[i] == s[i + k] {
                k += 1;
                *substrings.entry(&s[i..i + k]).or_default() += 1;
            }
        }

        substrings
            .into_iter()
            .filter_map(|(key, val)| if val >= 3 { Some(key.len()) } else { None })
            .max()
            .map(|len| len as i32)
            .unwrap_or(-1)
    }
}

fn main() {
    println!("Hello, world!");
}
