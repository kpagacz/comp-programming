// https://leetcode.com/problems/binary-number-with-alternating-bits/description/
pub struct Solution;

impl Solution {
    pub fn has_alternating_bits(mut n: i32) -> bool {
        let mut last_bit = n & 1;
        n >>= 1;

        while n > 0 {
            if n & 1 == last_bit {
                return false;
            } else {
                last_bit = n & 1;
                n >>= 1;
            }
        }
        true
    }
}

fn main() {
    println!("Hello, world!");
}
