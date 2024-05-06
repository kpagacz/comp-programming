// https://leetcode.com/contest/weekly-contest-395/problems/minimum-array-end/
pub struct Solution;

impl Solution {
    pub fn min_end(n: i32, x: i32) -> i64 {
        let mut n = n as i64 - 1;
        let x = x as i64;
        let mut answer = 0i64;
        let mut b = 1;

        while n > 0 {
            if (b & x) == 0 {
                answer |= (n & 1) * b;
                n >>= 1;
            }
            b <<= 1;
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
