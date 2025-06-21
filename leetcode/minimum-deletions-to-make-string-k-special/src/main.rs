// https://leetcode.com/problems/minimum-deletions-to-make-string-k-special/description/?envType=daily-question&envId=2025-06-21
pub struct Solution;

impl Solution {
    pub fn minimum_deletions(word: String, k: i32) -> i32 {
        let mut freq = [0; 26];

        for &b in word.as_bytes() {
            freq[(b - b'a') as usize] += 1;
        }

        let mut non_zero = freq
            .into_iter()
            .filter(|&count| count != 0)
            .collect::<Vec<_>>();
        non_zero.sort_unstable();

        let mut answer = i32::MAX;
        let mut prefix = 0;

        for i in 0..non_zero.len() {
            let min = non_zero[i];
            let max = non_zero[i] + k;
            let mut cost = prefix;
            for &freq in &non_zero[i + 1..] {
                cost += (freq - max).max(0);
            }
            answer = answer.min(cost);

            prefix += min;
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
