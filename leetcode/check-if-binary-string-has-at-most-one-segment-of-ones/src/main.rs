// https://leetcode.com/problems/check-if-binary-string-has-at-most-one-segment-of-ones/description/?envType=daily-question&envId=2026-03-06
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        s.split('0').filter(|chunk| !chunk.is_empty()).count() <= 1
    }
}

fn main() {
    let test_cases = [("1001", false), ("1100", true)];
    for (s, exp) in test_cases {
        let s = s.to_owned();
        println!("{} exp: {exp}", Solution::check_ones_segment(s));
    }
}
