// https://leetcode.com/problems/integer-break/description/
pub struct Solution {}

impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n == 2 {
            return 1;
        }
        if n == 3 {
            return 2;
        }
        let mut quotient: u32 = n as u32 / 3;
        let mut rest = 1;
        match n % 3 {
            0 => {}
            1 => {
                quotient -= 1;
                rest = 4;
            }
            2 => rest = 2,
            _ => unreachable!(),
        }

        3_i32.pow(quotient) * rest
    }
}

fn main() {
    println!("Hello, world!");
}
