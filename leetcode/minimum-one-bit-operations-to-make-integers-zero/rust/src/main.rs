// https://leetcode.com/problems/minimum-one-bit-operations-to-make-integers-zero/
pub struct Solution {}

impl Solution {
    pub fn minimum_one_bit_operations(n: i32) -> i32 {
        if n == 0 {
            return 0;
        }
        let leftmost_one = (0..=31).rev().find(|pos| (1 << pos) & n != 0).unwrap();
        let without_the_first_one = n ^ (1 << leftmost_one);
        (1 << (leftmost_one + 1)) - 1 - Self::minimum_one_bit_operations(without_the_first_one)
    }
}

fn main() {
    let test_cases = vec![3, 6];
    for n in test_cases {
        println!("{}", Solution::minimum_one_bit_operations(n));
    }
}
