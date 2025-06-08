// https://leetcode.com/problems/lexicographical-numbers/
pub struct Solution;

impl Solution {
    pub fn lexical_order(n: i32) -> Vec<i32> {
        let mut answer = Vec::with_capacity(n as usize);
        fn rec(current: i32, answer: &mut Vec<i32>, max: i32) {
            answer.push(current);

            if current * 10 <= max {
                rec(current * 10, answer, max);
            }

            if current % 10 != 9 && current < max {
                rec(current + 1, answer, max);
            }
        }
        rec(1, &mut answer, n);
        answer
    }
}

fn main() {
    println!("Hello, world!");
}
