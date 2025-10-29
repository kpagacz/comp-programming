// https://leetcode.com/problems/smallest-number-with-all-set-bits/description/?envType=daily-question&envId=2025-10-29
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn smallest_number(mut n: i32) -> i32 {
        let mut encountered_one = false;
        for i in (0..32).rev() {
            if n & (1 << i) > 0 {
                encountered_one = true;
            } else if encountered_one {
                n |= 1 << i;
            }
        }
        if n == 0 { 1 } else { n }
    }
}

fn main() {
    println!("Hello, world!");
}
