// https://leetcode.com/problems/longest-binary-subsequence-less-than-or-equal-to-k/description/?envType=daily-question&envId=2025-06-26
pub struct Solution;

impl Solution {
    pub fn longest_subsequence(s: String, k: i32) -> i32 {
        let mut count = 0;
        let mut current = 0;

        for (i, c) in s.chars().rev().enumerate() {
            match c {
                '1' if current + (1 << i) <= k => {
                    current += 1 << i;
                    count += 1;
                }
                '1' => {}
                '0' => count += 1,
                _ => unreachable!(),
            }
        }
        count
    }
}

fn main() {
    let test_cases = [("00101001", 1, 6)];
    for (s, k, exp) in test_cases {
        println!("Test: {s} k:{k}");
        println!(
            "{} | exp: {exp}",
            Solution::longest_subsequence(s.to_string(), k)
        );
    }
}
