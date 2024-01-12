// https://leetcode.com/problems/determine-if-string-halves-are-alike/description/
pub struct Solution;

const VOWELS: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let (first_half, second_half) = s.split_at(s.len() / 2);
        let first_half =
            first_half
                .chars()
                .fold(0, |num, c| if VOWELS.contains(&c) { num + 1 } else { num });
        let second_half =
            second_half
                .chars()
                .fold(0, |num, c| if VOWELS.contains(&c) { num + 1 } else { num });
        first_half == second_half
    }
}

fn main() {
    println!("Hello, world!");
}
