// https://leetcode.com/problems/repeated-substring-pattern/description/
pub struct Solution;

impl Solution {
    pub fn repeated_substring_pattern(s: String) -> bool {
        let s = s.as_bytes();
        let check_interval = |interval: usize| {
            if s.len() % interval != 0 {
                return false;
            }
            for start in 0..interval {
                let mut it = start;
                while it < s.len() {
                    if it != start && s[it] != s[it - interval] {
                        return false;
                    }
                    it += interval;
                }
            }
            true
        };

        (1..=s.len() / 2).any(check_interval)
    }
}

fn main() {
    println!("Hello, world!");
}
