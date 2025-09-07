// https://leetcode.com/problems/find-n-unique-integers-sum-up-to-zero/description/?envType=daily-question&envId=2025-09-07
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sum_zero(n: i32) -> Vec<i32> {
        let mut answer = Vec::with_capacity(n as usize);
        if n & 1 == 1 {
            answer.push(0);
        }

        for i in 1..=n / 2 {
            answer.push(i);
            answer.push(-i);
        }

        answer
    }
}

fn main() {
    println!("Hello, world!");
}
