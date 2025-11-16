// https://leetcode.com/problems/number-of-substrings-with-only-1s/description/?envType=daily-question&envId=2025-11-16
struct Solution;

const MOD: usize = 10usize.pow(9) + 7;
#[allow(dead_code)]
impl Solution {
    pub fn num_sub(s: String) -> i32 {
        let mut previous_zero = vec![0; s.len() + 2];
        let s = s.as_bytes();
        for i in 2..previous_zero.len() {
            if s[i - 2] == b'0' {
                previous_zero[i] = i - 1;
            } else {
                previous_zero[i] = previous_zero[i - 1];
            }
        }

        let mut answer = 0;
        for i in 2..previous_zero.len() {
            if i - 1 < s.len() && s[i - 1] == b'1' {
                continue;
            }
            let previous_zero = previous_zero[i];
            let ones = i - previous_zero - 1;
            let substrings = (ones * (ones + 1)) / 2;
            // println!("substrings: {substrings} i: {i} previous_zero: {previous_zero}");
            answer = (answer + substrings) % MOD;
        }
        answer as _
    }
}

fn main() {
    let test_cases = [("0110111", 9), ("000", 0)];
    for (s, exp) in test_cases {
        let s = s.to_owned();
        println!("{}  exp: {exp}", Solution::num_sub(s));
    }
}
