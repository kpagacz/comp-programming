// https://leetcode.com/problems/plus-one/description/?envType=daily-question&envId=2026-01-01
struct Solution;
#[allow(dead_code)]
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut carry = 1;
        let n = digits.len();
        let mut answer = Vec::with_capacity(n);
        for digit in digits.into_iter().rev() {
            let res = (carry + digit) % 10;
            carry = (carry + digit) / 10;
            answer.push(res);
        }
        if carry != 0 {
            answer.push(carry);
        }
        answer.reverse();
        answer
    }
}

fn main() {
    println!("Hello, world!");
}
