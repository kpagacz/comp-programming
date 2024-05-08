// https://leetcode.com/problems/base-7/description/
pub struct Solution;

impl Solution {
    pub fn convert_to_base7(mut num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }
        let mut answer = "".to_string();
        let negative = num < 0;
        if negative {
            num = -num;
        }

        while num > 0 {
            let digit = (num % 7).to_string();
            num /= 7;
            answer += &digit;
        }
        if negative {
            answer += "-";
        }

        answer.chars().rev().collect()
    }
}

fn main() {
    println!("Hello, world!");
}
