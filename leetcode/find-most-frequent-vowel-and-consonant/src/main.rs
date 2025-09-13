// https://leetcode.com/problems/find-most-frequent-vowel-and-consonant/description/?envType=daily-question&envId=2025-09-13
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_freq_sum(s: String) -> i32 {
        const VOWELS: [u8; 5] = [b'a', b'e', b'i', b'o', b'u'];
        let mut freqs = [0; 256];

        for letter in s.as_bytes().iter() {
            freqs[*letter as usize] += 1;
        }

        let mut max_vowel = 0;
        let mut max_consonant = 0;
        for (letter, freq) in freqs.into_iter().enumerate() {
            if VOWELS.contains(&(letter as u8)) {
                max_vowel = max_vowel.max(freq);
            } else {
                max_consonant = max_consonant.max(freq);
            }
        }
        max_vowel + max_consonant
    }
}

fn main() {
    println!("Hello, world!");
}
