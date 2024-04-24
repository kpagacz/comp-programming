// https://leetcode.com/problems/n-th-tribonacci-number/description/
pub struct Solution;

impl Solution {
    pub fn tribonacci(mut n: i32) -> i32 {
        let mut n0 = 0;
        let mut n1 = 1;
        let mut n2 = 1;

        while n > 0 {
            let new_n = n0 + n1 + n2;
            n0 = n1;
            n1 = n2;
            n2 = new_n;
            n -= 1;
        }
        n0
    }
}

fn main() {
    println!("Hello, world!");
}
