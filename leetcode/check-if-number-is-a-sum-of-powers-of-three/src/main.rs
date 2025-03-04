// https://leetcode.com/problems/check-if-number-is-a-sum-of-powers-of-three/
pub struct Solution;

impl Solution {
    pub fn check_powers_of_three(mut n: i32) -> bool {
        while n > 0 {
            if n % 3 == 2 {
                return false;
            }
            n /= 3;
        }
        true
    }
}

fn main() {
    println!("Hello, world!");
}
