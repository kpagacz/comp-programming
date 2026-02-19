// https://leetcode.com/problems/reverse-bits/description/?envType=daily-question&envId=2026-02-16
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn reverse_bits(n: i32) -> i32 {
        let mut answer = 0;
        for shift in 0..32 {
            if (n >> shift) & 1 == 1 {
                answer |= 1 << (31 - shift);
            }
        }
        answer
    }
}

fn main() {
    println!("Hello, world!");
}
