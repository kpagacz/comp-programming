// https://leetcode.com/problems/number-of-steps-to-reduce-a-number-in-binary-representation-to-one/description/
pub struct Solution;

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut have_one = false;
        let mut answer = 0;

        for c in s[1..].chars().rev() {
            match (c == '1', have_one) {
                (true, true) | (false, false) => answer += 1,
                (true, false) => {
                    have_one = true;
                    answer += 2;
                }
                (false, true) => answer += 2,
            }
        }

        answer + if have_one { 1 } else { 0 }
    }
}

fn main() {
    println!("Hello, world!");
}
