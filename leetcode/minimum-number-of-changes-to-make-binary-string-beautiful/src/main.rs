// https://leetcode.com/problems/minimum-number-of-changes-to-make-binary-string-beautiful/description/
pub struct Solution;

impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let s = s.as_bytes();

        s.chunks(2)
            .map(|chunk| if chunk[0] != chunk[1] { 1 } else { 0 })
            .sum()
    }
}

fn main() {
    let test_cases = ["1001", "10", "0000"];
    for s in test_cases {
        println!("{}", Solution::min_changes(s.to_string()));
    }
}
