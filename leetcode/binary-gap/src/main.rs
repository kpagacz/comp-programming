// https://leetcode.com/problems/binary-gap/description/?envType=daily-question&envId=2026-02-22
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        format!("{n:b}")
            .as_bytes()
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == b'1')
            .fold((0, 0), |acc, (i, _)| (acc.0.max(i - acc.1), i))
            .0 as _
    }
}

fn main() {
    println!("Hello, world!");
}
