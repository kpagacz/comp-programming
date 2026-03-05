// https://leetcode.com/problems/minimum-changes-to-make-alternating-binary-string/description/?envType=daily-question&envId=2026-03-05
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let mut even_ones = 0;
        let mut even_zeros = 0;
        let mut odd_ones = 0;
        let mut odd_zeros = 0;
        for (pos, c) in s.chars().enumerate() {
            match (pos % 2, c) {
                (0, '1') => even_ones += 1,
                (0, '0') => even_zeros += 1,
                (1, '1') => odd_ones += 1,
                (1, '0') => odd_zeros += 1,
                _ => unreachable!(),
            }
        }

        (odd_zeros + even_ones).min(odd_ones + even_zeros)
    }
}

fn main() {
    println!("Hello, world!");
}
