// https://leetcode.com/problems/maximum-69-number/submissions/?envType=daily-question&envId=2025-08-16
struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn maximum69_number(mut num: i32) -> i32 {
        let mut digits = vec![];
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }

        let last_six = digits.iter().rposition(|&num| num == 6);

        if let Some(last_six) = last_six {
            digits[last_six] = 9;
        }

        let mut answer = 0;
        while let Some(next_digit) = digits.pop() {
            answer *= 10;
            answer += next_digit;
        }
        answer
    }
}

fn main() {
    println!("Hello, world!");
}
