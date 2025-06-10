// https://leetcode.com/problems/maximum-difference-between-even-and-odd-frequency-i/description/
pub struct Solution;

impl Solution {
    pub fn max_difference(s: String) -> i32 {
        let mut freqs = [0; 26];

        for &c in s.as_bytes() {
            freqs[(c - b'a') as usize] += 1;
        }

        let max_odd = freqs
            .into_iter()
            .filter(|&val| val != 0 && val & 1 == 1)
            .max()
            .unwrap();
        let min_even = freqs
            .into_iter()
            .filter(|&val| val != 0 && val & 1 == 0)
            .min()
            .unwrap();
        max_odd - min_even
    }
}

fn main() {
    println!("Hello, world!");
}
