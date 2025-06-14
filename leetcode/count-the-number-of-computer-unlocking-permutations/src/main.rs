// https://leetcode.com/problems/count-the-number-of-computer-unlocking-permutations/description/
pub struct Solution;

impl Solution {
    pub fn count_permutations(complexity: Vec<i32>) -> i32 {
        let at_zero = complexity[0];

        for &c in &complexity[1..] {
            if c <= at_zero {
                return 0;
            }
        }

        let mut to_factorial = (complexity.len() - 1) as i64;
        const MOD: i64 = 10i64.pow(9) + 7;
        let mut answer = 1i64;
        while to_factorial > 0 {
            answer *= to_factorial;
            answer %= MOD;
            to_factorial -= 1;
        }
        answer as i32
    }
}

fn main() {
    println!("Hello, world!");
}
