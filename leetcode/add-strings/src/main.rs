// https://leetcode.com/problems/add-strings/description/
pub struct Solution;

impl Solution {
    pub fn add_strings(num1: String, num2: String) -> String {
        let num1 = num1.as_bytes();
        let num2 = num2.as_bytes();

        let digit = |c: u8| c - b'0';

        let len = num1.len().max(num2.len());

        let mut answer = vec![];
        let mut carry = 0;
        for i in 0..len {
            let digit1 = digit(*num1.get(num1.len() - 1 - i).unwrap_or(&b'0'));
            let digit2 = digit(*num2.get(num2.len() - 1 - i).unwrap_or(&b'0'));
            let sum = digit1 + digit2 + carry;
            let answer_digit = b'0' + sum % 10;
            carry = sum / 10;

            answer.push(answer_digit);
        }
        if carry != 0 {
            answer.push(carry + b'0');
        }

        String::from_utf8(answer.into_iter().rev().collect()).unwrap()
    }

    fn add_strings_with_its(num1: String, num2: String) -> String {
        let max_len = num1.len().max(num2.len());
        let mut num1 = num1.as_bytes().iter().rev().chain(std::iter::repeat(&b'0'));
        let mut num2 = num2.as_bytes().iter().rev().chain(std::iter::repeat(&b'0'));

        let mut answer = vec![];
        let mut carry = 0;

        let digit = |c: u8| c - b'0';
        for (&digit1, &digit2) in num1.zip(num2).take(max_len) {
            let sum = digit(digit1) + digit(digit2) + carry;
            answer.push((sum % 10) + b'0');
            carry = sum / 10;
        }

        if carry != 0 {
            answer.push(carry + b'0');
        }
        String::from_utf8(answer.into_iter().rev().collect()).unwrap()
    }
}

fn main() {
    println!(
        "{}",
        Solution::add_strings("11".to_string(), "123".to_string())
    );
}
