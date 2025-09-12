// https://leetcode.com/problems/vowels-game-in-a-string/description/?envType=daily-question&envId=2025-09-12
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn does_alice_win(s: String) -> bool {
        const VOWELS: [u8; 6] = [b'a', b'e', b'i', b'o', b'u', b'u'];
        let vowel_count = s.as_bytes().iter().fold(0, |vowel_count, letter| {
            if VOWELS.contains(letter) {
                vowel_count + 1
            } else {
                vowel_count
            }
        });
        vowel_count & 1 == 1 || (vowel_count > 0)
    }
}

fn main() {
    let test_cases = [("dcgykrp", false)];

    for (s, expected) in test_cases {
        println!(
            "{}   exp: {expected}",
            Solution::does_alice_win(s.to_owned())
        );
    }
}
