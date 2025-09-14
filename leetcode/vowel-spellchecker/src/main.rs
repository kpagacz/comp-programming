// https://leetcode.com/problems/vowel-spellchecker/description/?envType=daily-question&envId=2025-09-14
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn spellchecker(wordlist: Vec<String>, mut queries: Vec<String>) -> Vec<String> {
        use std::collections::HashMap;
        use std::collections::HashSet;

        let words_set: HashSet<&String> = HashSet::from_iter(&wordlist);
        let not_capitalized = wordlist.iter().rev().fold(HashMap::new(), |mut map, word| {
            map.insert(word.to_lowercase(), word.clone());
            map
        });
        let smartly_hashed = wordlist.iter().rev().fold(HashMap::new(), |mut map, word| {
            map.insert(Solution::smart_hash(&word.to_lowercase()), word.clone());
            map
        });

        queries.iter_mut().for_each(|word| {
            // If it's in wordlist, don't do anything
            if words_set.contains(word) {
                return;
            }

            // If the capitalization is different, return from wordlist
            if let Some(in_wordlist) = not_capitalized.get(&word.to_lowercase()) {
                *word = in_wordlist.clone();
                return;
            }

            // If vowels only don't match, return from wordlist
            if let Some(in_wordlist) =
                smartly_hashed.get(&Solution::smart_hash(&word.to_lowercase()))
            {
                *word = in_wordlist.clone();
                return;
            }

            *word = String::from("");
        });

        queries
    }

    fn smart_hash(word: &str) -> u64 {
        const VOWELS: [u8; 5] = [b'a', b'e', b'i', b'o', b'u'];
        const EXPONENT: u64 = 31;

        let mut power = 1;
        let mut hash = 0;

        for &letter in word.as_bytes() {
            if !VOWELS.contains(&letter) {
                let letter = letter - b'a';
                hash += letter as u64 * power;
            }
            power *= EXPONENT;
        }

        hash
    }
}

fn main() {
    let test_cases = [(
        vec!["KiTe", "kite", "hare", "Hare"],
        vec![
            "kite", "Kite", "KiTe", "Hare", "HARE", "Hear", "hear", "keti", "keet", "keto",
        ],
    )];

    for (wordlist, queries) in test_cases {
        println!(
            "{:?}",
            Solution::spellchecker(
                wordlist.into_iter().map(str::to_owned).collect(),
                queries.into_iter().map(str::to_owned).collect()
            )
        );
    }
}
