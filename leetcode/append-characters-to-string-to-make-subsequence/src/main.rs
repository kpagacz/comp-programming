// https://leetcode.com/problems/append-characters-to-string-to-make-subsequence/description/
pub struct Solution;

impl Solution {
    pub fn append_characters(s: String, t: String) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();

        let (mut matching, mut it) = (0, 0);

        while it < s.len() && matching < t.len() {
            if s[it] == t[matching] {
                matching += 1;
            }
            it += 1;
        }

        t.len() as i32 - matching as i32
    }
}

fn main() {
    let test_cases = [("scoaching", "coding"), ("abcde", "b"), ("z", "abc")];
    for (s, t) in test_cases {
        println!(
            "{}",
            Solution::append_characters(s.to_owned(), t.to_owned())
        );
    }
}
