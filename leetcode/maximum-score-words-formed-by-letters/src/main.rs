// https://leetcode.com/problems/maximum-score-words-formed-by-letters/description/
pub struct Solution;

impl Solution {
    pub fn max_score_words(words: Vec<String>, letters: Vec<char>, score: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let letters: HashMap<u8, i32> =
            letters.into_iter().fold(HashMap::new(), |mut map, letter| {
                *map.entry(letter as u8).or_default() += 1;
                map
            });

        fn score_words(
            words: &[String],
            letters: &HashMap<u8, i32>,
            scores: &[i32],
            bitmask: i32,
        ) -> i32 {
            let mut chars_count = HashMap::new();
            for (pos, word) in words.iter().rev().enumerate() {
                if (bitmask >> pos) & 1 == 1 {
                    word.chars().for_each(|c| {
                        *chars_count.entry(c as u8).or_insert(0) += 1;
                    });
                }
            }
            let mut score = 0;
            for (c, count) in chars_count {
                if *letters.get(&c).unwrap_or(&0) >= count {
                    score += count * scores[(c - b'a') as usize];
                } else {
                    return 0;
                }
            }
            score
        }

        let n = words.len();
        (0..(1 << n))
            .map(|mask| score_words(&words, &letters, &score, mask))
            .max()
            .unwrap()
    }
}

fn main() {
    let test_cases = [(
        vec!["dog", "cat", "dad", "good"],
        vec!['a', 'a', 'c', 'd', 'd', 'd', 'g', 'o', 'o'],
        vec![
            1, 0, 9, 5, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
    )];

    for (words, letters, score) in test_cases {
        println!(
            "{}",
            Solution::max_score_words(
                words.into_iter().map(str::to_string).collect(),
                letters,
                score
            )
        );
    }
}
