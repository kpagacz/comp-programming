// https://leetcode.com/problems/count-binary-substrings/description/?envType=daily-question&envId=2026-02-19
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn count_binary_substrings(s: String) -> i32 {
        let s = s.as_bytes();

        let mut answer = 0;
        let mut right = 0;
        let mut left = 0;

        while left < s.len() {
            // Go with the same char
            while right < s.len() && s[right] == s[left] {
                right += 1;
            }

            let first_count = right - left;
            // Go with a different char
            while right < s.len() && s[right] != s[left] {
                right += 1;
            }
            let second_count = right - left - first_count;

            let same = first_count.min(second_count);
            answer += same;

            left += first_count;
            right = left;
        }

        answer as _
    }
}

fn main() {
    let test_cases = [("00110011", 6), ("10101", 4), ("0111", 1)];
    for (s, exp) in test_cases {
        let s = s.to_owned();
        println!("{}, exp: {exp}", Solution::count_binary_substrings(s));
    }
}
