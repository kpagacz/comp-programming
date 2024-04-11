// https://leetcode.com/problems/remove-k-digits/description/
pub struct Solution;

impl Solution {
    pub fn remove_kdigits(num: String, mut k: i32) -> String {
        if num.len() == k as usize {
            return String::from("0");
        }

        let mut monotonic_stack = vec![];

        for c in num.chars() {
            while !monotonic_stack.is_empty() && *monotonic_stack.last().unwrap() > c && k > 0 {
                monotonic_stack.pop();
                k -= 1;
            }

            monotonic_stack.push(c);
        }
        while !monotonic_stack.is_empty() && k > 0 {
            monotonic_stack.pop();
            k -= 1;
        }
        let out = String::from_iter(monotonic_stack);
        let out = out.trim_start_matches('0');

        if out.is_empty() {
            "0".to_string()
        } else {
            out.to_string()
        }
    }
}

fn main() {
    let test_cases = [("1432219", 3), ("10200", 1), ("10", 2), ("010", 2)];
    for (num, k) in test_cases {
        println!("{}", Solution::remove_kdigits(num.to_string(), k));
    }
}
