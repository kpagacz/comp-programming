// https://leetcode.com/problems/number-complement/description/
pub struct Solution;

impl Solution {
    pub fn find_complement(num: i32) -> i32 {
        let mut digits = 0;

        let mut it = num;
        if it == 0 {
            digits = 1;
        } else {
            while it > 0 {
                digits += 1;
                it /= 2;
            }
        }

        let binary_mask = "1".repeat(digits);
        let mask = i32::from_str_radix(&binary_mask, 2).unwrap();

        num ^ mask
    }
}

fn main() {
    println!("Hello, world!");
}
