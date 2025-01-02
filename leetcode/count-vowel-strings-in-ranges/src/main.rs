// https://leetcode.com/problems/count-vowel-strings-in-ranges/description/
pub struct Solution;

impl Solution {
    pub fn vowel_strings(words: Vec<String>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut prefix = Vec::with_capacity(words.len() + 1);
        prefix.push(0);

        const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
        let mut acc = 0;
        for word in &words {
            if VOWELS.into_iter().any(|vowel| word.starts_with(vowel))
                && VOWELS.into_iter().any(|vowel| word.ends_with(vowel))
            {
                acc += 1;
            }
            prefix.push(acc);
        }

        queries
            .into_iter()
            .map(|query| {
                let (from, to) = (query[0] as usize, query[1] as usize);
                prefix[to + 1] - prefix[from]
            })
            .collect()
    }
}

fn main() {
    println!("Hello, world!");
}
