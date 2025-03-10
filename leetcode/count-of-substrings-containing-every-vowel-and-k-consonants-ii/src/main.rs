// https://leetcode.com/problems/count-of-substrings-containing-every-vowel-and-k-consonants-ii/description/?envType=daily-question&envId=2025-03-10
pub struct Solution;

// This problem has another solution with at_least(k) - at_least(k + 1) solution
// It's a trick worth remembering...
impl Solution {
    pub fn count_of_substrings(word: String, k: i32) -> i64 {
        const VOWELS: [u8; 5] = [b'a', b'e', b'i', b'o', b'u'];
        let mut vowels_counts = [0; 5];
        let mut consonants_count = 0;
        let word = word.as_bytes();
        let mut answer = 0i64;
        let mut left = 0usize;
        let mut next_consonant = 0usize;

        for (id, &c) in word.iter().enumerate() {
            match VOWELS.into_iter().position(|vowel| vowel == c) {
                Some(pos) => {
                    vowels_counts[pos] += 1;
                }
                None => consonants_count += 1,
            }

            while next_consonant <= id
                || (next_consonant < word.len() && VOWELS.contains(&word[next_consonant]))
            {
                next_consonant += 1;
            }

            while left < id
                && consonants_count >= k
                && vowels_counts.into_iter().all(|count| count > 0)
            {
                if consonants_count == k {
                    answer += (next_consonant - id) as i64;
                }

                match VOWELS.into_iter().position(|vowel| vowel == word[left]) {
                    Some(pos) => vowels_counts[pos] -= 1,
                    None => consonants_count -= 1,
                }
                left += 1;
            }
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
