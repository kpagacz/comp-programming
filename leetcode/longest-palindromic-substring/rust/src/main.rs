// https://leetcode.com/problems/longest-palindromic-substring/description/
pub struct Solution {}

use std::iter::once;
impl Solution {
    pub fn longest_palindrome(mut s: String) -> String {
        let palindrome_sizes = Self::manacher(&s);
        let mut longest_palindrome = (0, 0); // size, start
        (0..s.len()).for_each(|center| {
            let even_size = palindrome_sizes[center * 2] - 1;
            let odd_size = palindrome_sizes[center * 2 + 1] - 1;
            if even_size > longest_palindrome.0 {
                longest_palindrome = (even_size, center - even_size / 2);
            }
            if odd_size > longest_palindrome.0 {
                longest_palindrome = (odd_size, center - odd_size / 2);
            }
        });
        s.drain(longest_palindrome.1..longest_palindrome.1 + longest_palindrome.0)
            .collect()
    }

    fn manacher_odd(s: &str) -> Vec<usize> {
        let s: Vec<u8> = format!("^{}$", s).bytes().collect();
        let mut dp = vec![0; s.len()];
        let (mut l, mut r) = (1usize, 1usize);
        (1..(dp.len() - 1)).for_each(|center| {
            dp[center] = dp[center].max((r - center).min(dp[l + r - center]));
            while s[center - dp[center]] == s[center + dp[center]] {
                dp[center] += 1;
            }
            if center + dp[center] > r {
                l = center - dp[center];
                r = center + dp[center];
            }
        });

        dp.into_iter().skip(1).take(s.len() - 2).collect()
    }

    fn manacher(s: &str) -> Vec<usize> {
        let s = s
            .bytes()
            .flat_map(|c| [b'#', c])
            .chain(once(b'#'))
            .collect();
        let s = String::from_utf8(s).unwrap();
        Self::manacher_odd(&s)
    }
}
fn main() {
    println!(
        "{}",
        Solution::longest_palindrome(String::from("cbbbbdbabad"))
    );
    println!(
        "{}",
        Solution::longest_palindrome(String::from("dbbbbcaabaa"))
    );
}
