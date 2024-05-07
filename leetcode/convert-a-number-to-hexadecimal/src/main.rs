// https://leetcode.com/problems/convert-a-number-to-hexadecimal/description/
pub struct Solution;

impl Solution {
    pub fn to_hex(num: i32) -> String {
        let mut answer = String::from("");

        let base = 0xf;

        for i in [0, 4, 8, 12, 16, 20, 24, 28] {
            let digit = ((num >> i) & base) as u8;
            match digit {
                0..=9 => answer += &digit.to_string(),
                10..=15 => answer += &((b'a' + (digit - 10)) as char).to_string(),
                _ => unreachable!(),
            }
        }

        if answer == "00000000" {
            "0".to_string()
        } else {
            answer
                .chars()
                .rev()
                .skip_while(|c| *c == '0')
                .collect::<String>()
        }
    }
}

fn main() {
    let test_cases = [26];
    for num in test_cases {
        println!("{}", Solution::to_hex(num));
    }
}
