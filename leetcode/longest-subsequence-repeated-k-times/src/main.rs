// https://leetcode.com/problems/longest-subsequence-repeated-k-times/description/?envType=daily-question&envId=2025-06-27
pub struct Solution;

impl Solution {
    pub fn longest_subsequence_repeated_k(s: String, k: i32) -> String {
        let mut freq = [0; 26];
        for b in s.as_bytes() {
            freq[(b - b'a') as usize] += 1;
        }

        let mut candidates = String::new();
        for (i, count) in freq.into_iter().enumerate().rev() {
            for _ in 0..count / k {
                candidates.push((b'a' + i as u8) as char);
            }
        }

        fn is_repeated_k_times(s: &[u8], pattern: &[u8], mut k: i32) -> bool {
            let mut s_it = 0usize;
            let mut pattern_it = 0usize;
            while s_it < s.len() {
                if s[s_it] == pattern[pattern_it] {
                    pattern_it += 1;
                }
                if pattern_it == pattern.len() {
                    k -= 1;
                    pattern_it = 0;
                }
                s_it += 1;
            }
            k == 0
        }

        fn rec(s: &[u8], candidates: &[u8], pattern: &str, mask: i32, k: i32, best: &mut String) {
            for (i, character) in candidates.iter().enumerate() {
                if mask & (1 << i) == 0 {
                    let mut new_pattern = pattern.to_string();
                    new_pattern.push(*character as char);
                    if is_repeated_k_times(s, new_pattern.as_bytes(), k) {
                        if new_pattern.len() > best.len() {
                            *best = new_pattern.clone();
                        }
                        rec(s, candidates, &new_pattern, mask + (1 << i), k, best);
                    }
                }
            }
        }

        println!("{candidates}");
        let mut best = String::new();
        rec(s.as_bytes(), candidates.as_bytes(), "", 0, k, &mut best);
        best
    }
}

fn main() {
    let test_cases = [("bbabbabbbbabaababab", 3, "bbbb")];
    for (s, k, exp) in test_cases {
        println!(
            "{} | exp: {exp}",
            Solution::longest_subsequence_repeated_k(s.to_string(), k)
        );
    }
}
