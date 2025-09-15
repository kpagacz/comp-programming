// https://leetcode.com/problems/maximum-number-of-words-you-can-type/description/?envType=daily-question&envId=2025-09-15
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let broken_letters = broken_letters.as_bytes();
        text.split(' ')
            .filter(|word| {
                word.as_bytes()
                    .iter()
                    .all(|letter| !broken_letters.contains(letter))
            })
            .count() as _
    }
}

fn main() {
    println!("Hello, world!");
}
