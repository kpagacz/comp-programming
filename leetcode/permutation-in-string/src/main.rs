// https://leetcode.com/problems/permutation-in-string/description/
pub struct Solution;

impl Solution {
    pub fn check_inclusion(s1: String, s2: String) -> bool {
        let s1 = s1.as_bytes().iter().fold(vec![0; 26], |mut chars, &b| {
            chars[(b - b'a') as usize] += 1;
            chars
        });

        let mut left = 0;

        let s2 = s2.as_bytes();
        let mut current = vec![0; 26];
        for i in 0..s2.len() {
            let c = (s2[i] - b'a') as usize;
            current[c] += 1;

            while left < s2.len() && current[c] > s1[c] {
                let left_c = (s2[left] - b'a') as usize;
                current[left_c] -= 1;
                left += 1;
            }

            if current == s1 {
                return true;
            }
        }

        false
    }
}

fn main() {
    println!("Hello, world!");
}
