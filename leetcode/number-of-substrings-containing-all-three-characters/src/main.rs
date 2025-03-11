// https://leetcode.com/problems/number-of-substrings-containing-all-three-characters/description/?envType=daily-question&envId=2025-03-11
pub struct Solution;

impl Solution {
    pub fn number_of_substrings(s: String) -> i32 {
        let mut answer = 0;
        let mut left = 0usize;

        let mut counts = [0; 3];
        let n = s.len();
        for (pos, &c) in s.as_bytes().iter().enumerate() {
            counts[(c - b'a') as usize] += 1;

            while left < pos && counts[0] >= 1 && counts[1] >= 1 && counts[2] >= 1 {
                answer += n - pos;
                counts[(s.as_bytes()[left] - b'a') as usize] -= 1;
                left += 1;
            }
        }
        answer as _
    }
}

fn main() {
    println!("Hello, world!");
}
