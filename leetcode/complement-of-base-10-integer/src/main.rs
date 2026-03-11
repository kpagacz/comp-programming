// https://leetcode.com/problems/complement-of-base-10-integer/description/?envType=daily-question&envId=2026-03-11
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        let mut mask = 1;
        while mask <= n {
            mask <<= 1;
        }
        n ^ (mask - 1)
    }
}

fn main() {
    println!("Hello, world!");
}
