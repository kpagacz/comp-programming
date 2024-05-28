// https://leetcode.com/problems/get-equal-substrings-within-budget/description/
pub struct Solution;

impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let s = s.as_bytes();
        let t = t.as_bytes();
        let mut max_length = 0;
        let mut curr_cost = 0;

        let mut left = 0;

        for i in 0..s.len() {
            curr_cost += u8::abs_diff(s[i], t[i]) as i32;
            while left <= i && curr_cost > max_cost {
                curr_cost -= u8::abs_diff(s[left], t[left]) as i32;
                left += 1;
            }

            max_length = max_length.max(i as i32 - left as i32 + 1);
        }

        max_length
    }
}

fn main() {
    println!("Hello, world!");
}
