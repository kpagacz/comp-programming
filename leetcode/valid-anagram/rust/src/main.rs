// https://leetcode.com/problems/valid-anagram/description/
pub struct Solution {}

impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut freq = vec![0; 256];
        s.as_bytes().iter().for_each(|&c| freq[c as usize] += 1);
        t.as_bytes().iter().for_each(|&c| freq[c as usize] -= 1);

        freq.into_iter().all(|c| c == 0)
    }
}

fn main() {
    println!("Hello, world!");
}
