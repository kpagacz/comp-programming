// https://leetcode.com/problems/palindromic-substrings/description/
pub struct Solution;

impl Solution {
    pub fn count_substrings(s: String) -> i32 {
        let mut new_s = vec![b'#'; s.len() * 2 + 3];
        new_s[0] = b'^';
        for i in 0..s.len() {
            new_s[2 * i + 2] = s.as_bytes()[i];
        }
        *new_s.last_mut().unwrap() = b'$';
        let mut answer = 0;
        let mut left = 0;
        let mut right = 1;
        let mut palindrome_radii = vec![0; new_s.len()];
        for i in 1..new_s.len() - 1 {
            palindrome_radii[i] = (right - i).min(palindrome_radii[left + right - i]);
            while new_s[i - palindrome_radii[i]] == new_s[i + palindrome_radii[i]] {
                palindrome_radii[i] += 1;
            }
            if i + palindrome_radii[i] > right {
                left = i - palindrome_radii[i];
                right = i + palindrome_radii[i];
            }
            answer += palindrome_radii[i] / 2;
        }
        answer as _
    }
}

fn main() {
    let test_cases = ["abc", "aaa"];
    for s in test_cases {
        println!("{}", Solution::count_substrings(s.to_string()));
    }
}
