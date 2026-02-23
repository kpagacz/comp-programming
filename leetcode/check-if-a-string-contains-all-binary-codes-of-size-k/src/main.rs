// https://leetcode.com/problems/check-if-a-string-contains-all-binary-codes-of-size-k/description/?envType=daily-question&envId=2026-02-23
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        use std::collections::HashSet;

        let k = k as u32;
        let mut substrings = HashSet::new();

        s.as_bytes().windows(k as usize).for_each(|window| {
            substrings.insert(window.to_vec());
        });

        substrings.len() == 2usize.pow(k)
    }
}

fn main() {
    println!("Hello, world!");
}
