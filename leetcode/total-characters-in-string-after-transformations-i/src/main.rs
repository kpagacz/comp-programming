// https://leetcode.com/problems/total-characters-in-string-after-transformations-i/description/?envType=daily-question&envId=2025-05-13
pub struct Solution;

impl Solution {
    pub fn length_after_transformations(s: String, t: i32) -> i32 {
        const MOD: i32 = 10i32.pow(9) + 7;
        let mut freq = [0; 26];
        for &b in s.as_bytes() {
            freq[(b - b'a') as usize] += 1;
        }

        for _ in 0..t {
            freq.rotate_right(1);
            freq[1] += freq[0];
            freq[1] %= MOD;
        }
        freq.into_iter().fold(0i32, |acc, num| (acc + num) % MOD)
    }
}

fn main() {
    println!("Hello, world!");
}
