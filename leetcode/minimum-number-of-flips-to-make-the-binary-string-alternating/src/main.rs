// https://leetcode.com/problems/minimum-number-of-flips-to-make-the-binary-string-alternating/description/?envType=daily-question&envId=2026-03-07
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn min_flips(s: String) -> i32 {
        let mut odd_ones = 0;
        let mut even_ones = 0;
        let mut odd_zeros = 0;
        let mut even_zeros = 0;
        for (pos, c) in s.chars().enumerate() {
            match (pos & 1, c) {
                (0, '0') => even_zeros += 1,
                (0, '1') => even_ones += 1,
                (1, '0') => odd_zeros += 1,
                (1, '1') => odd_ones += 1,
                _ => unreachable!(),
            }
        }

        let mut answer = (even_zeros + odd_ones).min(even_ones + odd_zeros);
        if s.len() & 1 == 0 {
            // Because the even string when shifted does not change the counts
            return answer;
        }

        for i in 0..s.len() {
            let mut shifted_even_ones = odd_ones;
            let mut shifted_even_zeros = odd_zeros;
            let mut shifted_odd_ones = even_ones;
            let mut shifted_odd_zeros = even_zeros;

            if &s[i..i + 1] == "1" {
                shifted_odd_ones -= 1;
                shifted_even_ones += 1;
            }
            if &s[i..i + 1] == "0" {
                shifted_odd_zeros -= 1;
                shifted_even_zeros += 1;
            }

            answer = answer
                .min(shifted_even_zeros + shifted_odd_ones)
                .min(shifted_even_ones + shifted_odd_zeros);

            even_ones = shifted_even_ones;
            even_zeros = shifted_even_zeros;
            odd_ones = shifted_odd_ones;
            odd_zeros = shifted_odd_zeros;
        }

        answer
    }
}

fn main() {
    let test_cases = [("111000", 2), ("010", 0), ("1110", 1), ("01001001101", 2)];

    for (s, exp) in test_cases {
        let s = s.to_string();
        println!("{} exp: {exp}", Solution::min_flips(s));
    }
}
